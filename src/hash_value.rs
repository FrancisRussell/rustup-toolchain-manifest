use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use thiserror::Error;

/// Parse errors for `HashValue`
#[derive(Clone, Copy, Debug, Error)]
pub enum ParseError {
    /// An input character was invalid (not-hexadecimal)
    #[error("Invalid byte: {0}")]
    InvalidByte(u8),

    /// The hash was not a mutiple of 8-bits (had an odd number of characters)
    #[error("Not a multiple of 8 bits")]
    NotOctetSized,
}

/// A dynamically sized hash used to represent Git SHAs and digest values
#[derive(Default, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HashValue {
    bytes: Vec<u8>,
}

impl HashValue {
    fn nibble_to_ascii(c: u8) -> u8 {
        match c {
            0..=9 => b'0' + c,
            10..=15 => b'a' + (c - 10),
            _ => panic!("Value not in range 0-15: {}", c),
        }
    }

    fn ascii_to_nibble(c: u8) -> Result<u8, ParseError> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            _ => Err(ParseError::InvalidByte(c)),
        }
    }

    /// Returns the value as ASCII characters
    fn to_ascii(&self) -> Vec<u8> {
        let mut characters = vec![0u8; self.bytes.len() * 2];
        for (idx, character) in self.bytes.iter().enumerate() {
            let high = character >> 4;
            let low = character & 0xf;
            characters[idx * 2] = Self::nibble_to_ascii(high);
            characters[idx * 2 + 1] = Self::nibble_to_ascii(low);
        }
        characters
    }

    /// Converts from binary (not ASCII)
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> HashValue {
        HashValue { bytes: bytes.to_vec() }
    }
}

impl AsRef<[u8]> for HashValue {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl std::fmt::Debug for HashValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "\"{}\"", self)
    }
}

impl std::fmt::Display for HashValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let ascii = self.to_ascii();
        let ascii = unsafe { std::str::from_utf8_unchecked(&ascii) };
        formatter.write_str(ascii)
    }
}

impl Serialize for HashValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let ascii = self.to_ascii();
            let ascii = unsafe { std::str::from_utf8_unchecked(&ascii) };
            serializer.serialize_str(ascii)
        } else {
            serializer.serialize_bytes(&self.bytes)
        }
    }
}

impl<'a> Deserialize<'a> for HashValue {
    fn deserialize<D>(deserializer: D) -> Result<HashValue, D::Error>
    where
        D: Deserializer<'a>,
    {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            Ok(HashValue::from_str(s.as_str()).map_err(serde::de::Error::custom)?)
        } else {
            let bytes = <Vec<u8>>::deserialize(deserializer)?;
            Ok(HashValue { bytes })
        }
    }
}

impl std::str::FromStr for HashValue {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<HashValue, ParseError> {
        let string = string.as_bytes();
        let length = string.len();
        if length % 2 == 0 {
            let mut bytes = vec![0u8; length / 2];
            for (idx, byte) in bytes.iter_mut().enumerate() {
                let high = Self::ascii_to_nibble(string[idx * 2])?;
                let low = Self::ascii_to_nibble(string[idx * 2 + 1])?;
                *byte = (high << 4) | low;
            }
            Ok(HashValue { bytes })
        } else {
            Err(ParseError::NotOctetSized)
        }
    }
}
