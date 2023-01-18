use std::{collections::HashMap, fmt::Debug, hash::Hash, io::Cursor};

use enum_dispatch::enum_dispatch;

use crate::{
    error::{DeserializeError, Error},
    scoped_stack_entry::ScopedStackEntry,
};

use self::{
    array_property::ArrayProperty,
    enum_property::EnumProperty,
    int_property::{
        BoolProperty, ByteProperty, DoubleProperty, FloatProperty, Int16Property, Int64Property,
        Int8Property, IntProperty, UInt16Property, UInt32Property, UInt64Property,
    },
    map_property::MapProperty,
    set_property::SetProperty,
    str_property::StrProperty,
    struct_property::StructProperty,
    unknown_property::UnknownProperty,
};

pub mod array_property;
pub mod enum_property;
pub mod int_property;
pub mod map_property;
pub mod set_property;
pub mod str_property;
pub mod struct_property;
pub mod struct_types;
pub mod unknown_property;

#[macro_export]
macro_rules! cast {
    ($namespace:ident, $type:ident, $field:expr) => {
        match $field {
            $namespace::$type(e) => Some(e),
            _ => None,
        }
    };
}

#[enum_dispatch]
pub trait PropertyTrait: Debug + Clone + PartialEq + Eq + Hash {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error>;
}

#[enum_dispatch(PropertyTrait)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(tag = "type"))]
pub enum Property {
    Int8Property,
    ByteProperty,
    Int16Property,
    UInt16Property,
    IntProperty,
    UInt32Property,
    Int64Property,
    UInt64Property,
    FloatProperty,
    DoubleProperty,
    BoolProperty,
    EnumProperty,
    StrProperty,
    StructProperty,
    ArrayProperty,
    SetProperty,
    MapProperty,

    UnknownProperty,
}

impl Property {
    pub fn new(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
        value_type: &str,
        include_header: bool,
        suggested_length: Option<u64>,
    ) -> Result<Self, Error> {
        let _stack_entry = ScopedStackEntry::new(properties_stack, value_type.to_string());
        match value_type {
            "Int8Property" => Ok(Int8Property::read(cursor, include_header)?.into()),
            "ByteProperty" => Ok(ByteProperty::read(cursor, include_header)?.into()),
            "Int16Property" => Ok(Int16Property::read(cursor, include_header)?.into()),
            "UInt16Property" => Ok(UInt16Property::read(cursor, include_header)?.into()),
            "IntProperty" => Ok(IntProperty::read(cursor, include_header)?.into()),
            "UInt32Property" => Ok(UInt32Property::read(cursor, include_header)?.into()),
            "Int64Property" => Ok(Int64Property::read(cursor, include_header)?.into()),
            "UInt64Property" => Ok(UInt64Property::read(cursor, include_header)?.into()),
            "FloatProperty" => Ok(FloatProperty::read(cursor, include_header)?.into()),
            "DoubleProperty" => Ok(DoubleProperty::read(cursor, include_header)?.into()),
            "BoolProperty" => Ok(BoolProperty::read(cursor, include_header)?.into()),
            "EnumProperty" => Ok(EnumProperty::read(cursor)?.into()),
            "StrProperty" => Ok(StrProperty::read(cursor, include_header)?.into()),
            "StructProperty" => {
                if !include_header {
                    let struct_path = properties_stack.join(".");
                    let Some(hint) = hints.get(&struct_path) else {
                        return Err(DeserializeError::MissingHint(value_type.to_string(), struct_path, cursor.position()).into());
                    };

                    return Ok(StructProperty::read_with_type_name(
                        cursor,
                        hints,
                        properties_stack,
                        hint,
                    )?
                    .into());
                }

                Ok(StructProperty::read_with_header(cursor, hints, properties_stack)?.into())
            }
            "ArrayProperty" => Ok(ArrayProperty::read(cursor, hints, properties_stack)?.into()),
            "SetProperty" => Ok(SetProperty::read(cursor, hints, properties_stack)?.into()),
            "MapProperty" => Ok(MapProperty::read(cursor, hints, properties_stack)?.into()),
            _ => {
                if include_header {
                    return Ok(
                        UnknownProperty::read_with_header(cursor, value_type.to_string())?.into(),
                    );
                }

                if let Some(suggested_length) = suggested_length {
                    return Ok(UnknownProperty::read_with_length(
                        cursor,
                        value_type.to_string(),
                        suggested_length,
                    )?
                    .into());
                }
                panic!("Invalid property creation call!");
            }
        }
    }
}

macro_rules! inner_traits {
    ($($property:ident),*) => {
        impl Debug for Property {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$property(arg0) => f.debug_tuple(stringify!($property)).field(arg0).finish(),
                    )*
                }
            }
        }

        impl Clone for Property {
            fn clone(&self) -> Self {
                match self {
                    $(
                        Self::$property(arg0) => Self::$property(arg0.clone()),
                    )*
                }
            }
        }

        impl PartialEq for Property {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    $(
                        (Self::$property(l0), Self::$property(r0)) => l0 == r0,
                    )*
                    _ => false
                }
            }
        }

        impl Eq for Property {}

        impl Hash for Property {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                core::mem::discriminant(self).hash(state);
            }
        }
    };
}

inner_traits!(
    Int8Property,
    ByteProperty,
    Int16Property,
    UInt16Property,
    IntProperty,
    UInt32Property,
    Int64Property,
    UInt64Property,
    FloatProperty,
    DoubleProperty,
    BoolProperty,
    EnumProperty,
    StrProperty,
    StructProperty,
    ArrayProperty,
    SetProperty,
    MapProperty,
    UnknownProperty
);
