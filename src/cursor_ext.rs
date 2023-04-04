use std::{
    io::{Cursor, Read, Write},
    mem::size_of,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::{DeserializeError, Error};

pub trait CursorExt {
    fn read_string(&mut self) -> Result<String, Error>;
    fn write_string(&mut self, v: &str) -> Result<(), Error>;
}

impl CursorExt for Cursor<Vec<u8>> {
    fn read_string(&mut self) -> Result<String, Error> {
        let len = self.read_i32::<LittleEndian>()?;
        if len == i32::MIN {
            return Err(DeserializeError::InvalidString(len).into());
        }

        if !(-131072..=131072).contains(&len) {
            return Err(DeserializeError::InvalidString(len).into());
        }

        if len == 0 {
            return Ok("".to_string());
        }

        if len < 0 {
            let len = -len;

            let len = len * size_of::<u16>() as i32 - 2;
            let mut buf = vec![0u8; len as usize];
            self.read_exact(&mut buf)?;

            let string = String::from_utf16(
                &buf.chunks(2)
                    .map(|e| u16::from_le_bytes([e[0], e[1]]))
                    .collect::<Vec<_>>(),
            )?;

            self.read_exact(&mut [0u8; 2])?;

            Ok(string)
        } else {
            let mut buf = vec![0u8; len as usize - 1];
            self.read_exact(&mut buf)?;
            self.read_exact(&mut [0u8; 1])?;

            Ok(String::from_utf8(buf).unwrap_or_else(|_| String::from("None")))
        }
    }

    fn write_string(&mut self, v: &str) -> Result<(), Error> {
        if v.is_ascii() {
            // ASCII strings do not require encoding
            let len = v.len() + 1;
            self.write_i32::<LittleEndian>(len as i32)?;
            let _ = self.write(v.as_bytes())?;
            let _ = self.write(&[0u8; 1])?;
        } else {
            // Perform UTF-16 encoding when non-ASCII characters are detected
            let words: Vec<u16> = v.encode_utf16().collect();
            let len = words.len() + 1;
            self.write_i32::<LittleEndian>(-(len as i32))?;
            for word in words {
                self.write_u16::<LittleEndian>(word)?;
            }
            self.write_u16::<LittleEndian>(0u16)?;
        }
        Ok(())
    }
}
