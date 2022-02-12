use crate::chunk_type::ChunkType;
use anyhow::{anyhow, Result};
use crc::{CRC_32_CKSUM, CRC_32_ISO_HDLC};
use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

pub struct Chunk {
    _length: u32,
    _type: ChunkType,
    _data: Vec<u8>,
    _crc: u32,
}

impl Chunk {
    pub fn new(_type: ChunkType, _data: Vec<u8>) -> Self {
        let mut c = Chunk {
            _length: _data.len() as u32,
            _crc: 0,
            _type,
            _data,
        };
        let rest = c.as_bytes();
        let (_, rest) = rest.split_at(4);
        let (rest, _) = rest.split_at(rest.len() - 4);
        c._crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(rest);
        c
    }
    pub fn length(&self) -> u32 {
        self._length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self._type
    }
    pub fn data(&self) -> &[u8] {
        self._data.as_ref()
    }
    pub fn crc(&self) -> u32 {
        self._crc
    }
    pub fn data_as_string(&self) -> std::result::Result<String, FromUtf8Error> {
        String::from_utf8(self._data.clone())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self._length
            .to_be_bytes()
            .iter()
            .cloned()
            .chain(self._type.bytes().iter().cloned())
            .chain(self._data.iter().cloned())
            .chain(self._crc.to_be_bytes().iter().cloned())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            Err(anyhow!("Invalid Chunk String {:?} : Too Short", value))
        } else {
            let (len, rest) = value.split_at(4);
            if bytes_to_u32(len) > rest.len() as u32 {
                return Err(anyhow!("Invalid Chunk String {:?} : Too Short", value));
            }
            let (rest, _) = rest.split_at(bytes_to_u32(len) as usize + 8);
            let (rest, crc) = rest.split_at(bytes_to_u32(len) as usize + 4);

            let _crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(rest);
            if bytes_to_u32(crc) != _crc {
                Err(anyhow!(
                    "Invalid Chunk String {:?} : Wrong CRC {} , Should Be {}",
                    value,
                    bytes_to_u32(crc),
                    _crc
                ))
            } else {
                let (chunk_type, data) = rest.split_at(4);

                Ok(Chunk {
                    _length: bytes_to_u32(len),
                    _type: ChunkType::try_from(<[u8; 4]>::try_from(chunk_type)?).unwrap(),
                    _data: data.to_vec(),
                    _crc,
                })
            }
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "s")
    }
}

fn bytes_to_u32(value: &[u8]) -> u32 {
    ((value[0] as u32) << 24)
        + ((value[1] as u32) << 16)
        + ((value[2] as u32) << 8)
        + ((value[3] as u32) << 0)
}
fn u32_to_bytes(value: u32) -> [u8; 4] {
    [
        (value & 0xFF000000) as u8,
        (value & 0x00FF0000) as u8,
        (value & 0x0000FF00) as u8,
        (value & 0x000000FF) as u8,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }
    #[test]
    fn test_valid_chunk_from_bytes_str() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = message_bytes.iter().copied().collect();

        let chunk = Chunk::new(ChunkType::from_str("RuSt").unwrap(), chunk_data);

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
