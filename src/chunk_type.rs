use anyhow::{anyhow, Result};
use core::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct ChunkType {
    _data: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self._data.clone()
    }
    pub fn is_valid(&self) -> bool {
        self._data[0].is_ascii_alphabetic()
            && self._data[1].is_ascii_alphabetic()
            && self._data[2].is_ascii_alphabetic()
            && self._data[3].is_ascii_alphabetic()
            && self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        self._data[0] & 32 == 0
    }
    pub fn is_public(&self) -> bool {
        self._data[1] & 32 == 0
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self._data[2] & 32 == 0
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self._data[3] & 32 != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { _data: value })
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 4 {
            Err(anyhow!("Invalid Chunk Type String {} : Too Short", s))
        } else {
            if s.as_bytes()[0].is_ascii_alphabetic()
                && s.as_bytes()[1].is_ascii_alphabetic()
                && s.as_bytes()[2].is_ascii_alphabetic()
                && s.as_bytes()[3].is_ascii_alphabetic()
            {
                Ok(ChunkType {
                    _data: [
                        s.as_bytes()[0],
                        s.as_bytes()[1],
                        s.as_bytes()[2],
                        s.as_bytes()[3],
                    ],
                })
            } else {
                Err(anyhow!(
                    "Invalid Chunk Type String {} : Invalid Character",
                    s
                ))
            }
        }
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self._data[0] as char,
            self._data[1] as char,
            self._data[2] as char,
            self._data[3] as char
        )
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self._data[0] & 32 == other._data[0] & 32
            && self._data[1] & 32 == other._data[1] & 32
            && self._data[2] & 32 == other._data[2] & 32
            && self._data[3] & 32 == other._data[3] & 32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
