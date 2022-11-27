use generic_array::typenum::{self, op, U2};
use generic_array::{ArrayLength, GenericArray};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Error)]
pub enum ParseError {
    #[error("Invalid byte: {0}")]
    InvalidByte(u8),

    #[error("Invalid length: {0}")]
    InvalidLength(usize),
}

#[derive(Default, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HashValue<N: ArrayLength<u8>>
where
    N::ArrayType: Copy,
{
    bytes: GenericArray<u8, N>,
}

impl<N> HashValue<N>
where
    N: ArrayLength<u8>,
    N::ArrayType: Copy,
{
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

    fn to_ascii<O>(&self) -> GenericArray<u8, op!(N * U2)>
    where
        N: std::ops::Mul<U2, Output = O>,
        O: ArrayLength<u8>,
    {
        let mut characters = GenericArray::<u8, op!(N * U2)>::default();
        for (idx, character) in self.bytes.iter().enumerate() {
            let high = character >> 4;
            let low = character & 0xf;
            characters[idx * 2] = Self::nibble_to_ascii(high);
            characters[idx * 2 + 1] = Self::nibble_to_ascii(low);
        }
        characters
    }
}

impl<N> std::fmt::Debug for HashValue<N>
where
    HashValue<N>: std::fmt::Display,
    N: ArrayLength<u8>,
    N::ArrayType: Copy,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "\"{}\"", self)
    }
}

impl<N, O> std::fmt::Display for HashValue<N>
where
    N: ArrayLength<u8>,
    N::ArrayType: Copy,
    N: std::ops::Mul<U2, Output = O>,
    O: ArrayLength<u8>,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let ascii = self.to_ascii();
        let ascii = unsafe { std::str::from_utf8_unchecked(&ascii) };
        formatter.write_str(ascii)
    }
}

impl<N, O> Serialize for HashValue<N>
where
    N: ArrayLength<u8>,
    N::ArrayType: Copy,
    N: std::ops::Mul<U2, Output = O>,
    O: ArrayLength<u8>,
{
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

impl<'a, N> Deserialize<'a> for HashValue<N>
where
    N: ArrayLength<u8> + typenum::ToInt<usize>,
    N::ArrayType: Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<HashValue<N>, D::Error>
    where
        D: Deserializer<'a>,
    {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            Ok(HashValue::from_str(s.as_str()).map_err(serde::de::Error::custom)?)
        } else {
            let bytes = <GenericArray<u8, N>>::deserialize(deserializer)?;
            Ok(HashValue { bytes })
        }
    }
}

impl<N> std::str::FromStr for HashValue<N>
where
    N: ArrayLength<u8> + typenum::ToInt<usize>,
    N::ArrayType: Copy,
{
    type Err = ParseError;

    fn from_str(string: &str) -> Result<HashValue<N>, ParseError> {
        let string = string.as_bytes();
        let length = string.len();
        if length != N::to_int() * 2 {
            Err(ParseError::InvalidLength(length))
        } else {
            let mut bytes = GenericArray::<u8, N>::default();
            for (idx, byte) in bytes.iter_mut().enumerate() {
                let high = Self::ascii_to_nibble(string[idx * 2])?;
                let low = Self::ascii_to_nibble(string[idx * 2 + 1])?;
                *byte = (high << 4) | low;
            }
            Ok(HashValue { bytes })
        }
    }
}

pub type Hash160 = HashValue<typenum::U20>;
pub type Hash256 = HashValue<typenum::U32>;
