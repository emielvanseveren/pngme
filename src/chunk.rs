#![allow(unused_variables)]
use crate::chunk_type::ChunkType;
use crate::Result;
use std::convert::TryFrom;
use std::fmt;

/*
Fields of a chunk.
  length: u32;
  chunkType: ChunkType;
  data: [u8; length];
  crc: u32;
*/

pub struct Chunk {
  //  length: u32,
  chunk_type: crate::chunk_type::ChunkType,
  data: [u8],
}

impl Chunk {
  pub fn length(&self) -> u32 {
    0
  }
  pub fn chunk_type(&self) -> &ChunkType {
    &self.chunk_type
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }

  pub fn crc(&self) -> u32 {
    // crc logic
    0
  }

  pub fn data_as_string(&self) -> Result<String> {
    Ok(String::from("data"))
  }

  pub fn as_bytes(&self) -> Vec<u8> {
    Vec::new()
  }
}

// impl TryFrom for Chunk {}
impl fmt::Display for Chunk {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.chunk_type, self.data.len())
  }
}

fn main() {
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
}
