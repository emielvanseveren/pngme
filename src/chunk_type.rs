use crate::Error;
use std::{
  convert::TryFrom,
  fmt::{Display, Formatter},
  str::FromStr,
};

/*
Fields of a chunk.
  length: u32;
  chunkType: ChunkType;
  data: [u8; length];
  crc: u32;
*/

#[derive(Debug)]
pub enum ChunkTypeError {
  ByteLengthError(usize),
  InvalidCharacter,
}
impl std::error::Error for ChunkTypeError {}

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
  bytes: [u8; 4],
}

impl ChunkType {
  pub fn bytes(&self) -> &[u8] {
    return &self.bytes;
  }

  // Byte must be between the lower-case and upper-case ASCII range.
  // Reserved bit must be valid.
  pub fn is_valid(&self) -> bool {
    let valid_chars = self
      .bytes
      .iter()
      .all(|&b| (b >= b'a' && b <= b'z' || (b >= b'A' && b <= b'Z')));
    valid_chars && self.is_reserved_bit_valid()
  }

  // A type code is critical if bit 5 of the first byte is 0.
  pub fn is_critical(&self) -> bool {
    (self.bytes[0] & 0x20) != 0x20
  }

  // A type code is public if bit 5 of the second byte is 0.
  pub fn is_public(&self) -> bool {
    (self.bytes[1] & 0x20) != 0x20
  }

  // Bit 5 of the third byte is reserved and must be 0.
  pub fn is_reserved_bit_valid(&self) -> bool {
    (self.bytes[2] & 0x20) != 0x20
  }

  // A type code is safe to copy if bit 5 of the fourth byte is 1.
  pub fn is_safe_to_copy(&self) -> bool {
    (self.bytes[3] & 0x20) == 0x20
  }
}

impl TryFrom<[u8; 4]> for ChunkType {
  type Error = Error;
  fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
    // todo
    Ok(Self { bytes: value })
  }
}

impl FromStr for ChunkType {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let bytes = s.as_bytes();

    if bytes.len() != 4 {
      return Err(Box::new(ChunkTypeError::ByteLengthError(bytes.len())));
    }

    let valid_chars = bytes
      .iter()
      .all(|&b| (b >= b'a' && b <= b'z' || (b >= b'A' && b <= b'Z')));

    if !valid_chars {
      return Err(Box::new(ChunkTypeError::InvalidCharacter));
    }

    let sized: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
    Ok(ChunkType::try_from(sized)?)
  }
}
impl Display for ChunkType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = std::str::from_utf8(&self.bytes).map_err(|_| std::fmt::Error)?;
    write!(f, "{}", s)
  }
}

impl Display for ChunkTypeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ChunkTypeError::ByteLengthError(actual) => {
        write!(f, "Expected 4 bytes but received {}", actual)
      }
      ChunkTypeError::InvalidCharacter => {
        write!(f, "Input contains one or more invalid characters")
      }
    }
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
    println!(
      "SHOULD be public..........................................................................\n"
    );
    let chunk = ChunkType::from_str("RUSt").unwrap();
    assert!(chunk.is_public());
  }

  #[test]
  pub fn test_chunk_type_is_not_public() {
    println!("SHOULD not be public..........................................................................\n");
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
