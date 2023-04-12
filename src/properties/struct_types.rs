use std::{fmt::Display, hash::Hash};

use ordered_float::OrderedFloat;

/// A struct that stores a vector.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector {
    /// X coordinate.
    pub x: OrderedFloat<f32>,
    /// Y coordinate.
    pub y: OrderedFloat<f32>,
    /// Z coordinate.
    pub z: OrderedFloat<f32>,
}

impl Vector {
    /// Creates a new `Vector` instance.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let x = OrderedFloat::from(x);
        let y = OrderedFloat::from(y);
        let z = OrderedFloat::from(z);
        Vector { x, y, z }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x, self.y, self.z)
    }
}

/// A struct that stores a rotator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rotator {
    /// Euclidean pitch.
    pub pitch: OrderedFloat<f32>,
    /// Euclidean yaw.
    pub yaw: OrderedFloat<f32>,
    /// Euclidean roll.
    pub roll: OrderedFloat<f32>,
}

impl Rotator {
    /// Creates a new `Rotator` instance.
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        let pitch = OrderedFloat::from(pitch);
        let yaw = OrderedFloat::from(yaw);
        let roll = OrderedFloat::from(roll);
        Rotator { pitch, yaw, roll }
    }
}

impl Display for Rotator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pitch: {} yaw: {} roll: {}",
            self.pitch, self.yaw, self.roll
        )
    }
}

/// A struct that stores a quaternion.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quat {
    /// X component.
    pub x: OrderedFloat<f32>,
    /// Y component.
    pub y: OrderedFloat<f32>,
    /// Z component.
    pub z: OrderedFloat<f32>,
    /// Real component.
    pub w: OrderedFloat<f32>,
}

impl Quat {
    /// Creates a new `Quat` instance.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        let x = OrderedFloat::from(x);
        let y = OrderedFloat::from(y);
        let z = OrderedFloat::from(z);
        let w = OrderedFloat::from(w);
        Quat { x, y, z, w }
    }
}

impl Display for Quat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {} z: {} w: {}", self.x, self.y, self.z, self.w)
    }
}

/// A struct that stores a date and time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DateTime {
    /// Ticks.
    pub ticks: u64,
}

impl DateTime {
    /// Creates a new `DateTime` instance.
    pub fn new(ticks: u64) -> Self {
        DateTime { ticks }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ticks: {}", self.ticks)
    }
}

/// A struct that stores a 2D integer point.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntPoint {
    /// X value.
    pub x: i32,
    /// Y value.
    pub y: i32,
}

impl IntPoint {
    /// Creates a new `IntPoint` instance.
    pub fn new(x: i32, y: i32) -> Self {
        IntPoint { x, y }
    }
}

impl Display for IntPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}
