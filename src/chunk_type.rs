use std::fmt;
use std::str::FromStr;
use crate::{Error, Result};

/// A validated PNG chunk. See PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
	chunk_type: [u8; 4]
}
impl ChunkType {
	// Returns the raw bytes contained in this chunk
	pub fn bytes(&self) -> [u8; 4] {
		self.chunk_type
	}

	// Returns the property state of the first byte as described in the PNG spec
	pub fn is_critical(&self) -> bool {
		// TODO: Check if there is a better way to do this, even if this works
		self.chunk_type[0] > u8::try_from('A').unwrap() && self.chunk_type[0] < u8::try_from('Z').unwrap()
	}

	// Returns the property state of the second byte as described in the PNG spec
	pub fn is_public(&self) -> bool {
		todo!(3);
	}

	// Returns the property state of the third byte as described in the PNG spec
	pub fn is_reserved_bit_valid(&self) -> bool {
		todo!(4);
	}

	// Returns the property state of the fourth byte as described in the PNG spec
	pub fn is_safe_to_copy(&self) -> bool {
		todo!(5);
	}

	// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z
	// Note that this chunk type should always be valid as it is validated during construction
	pub fn is_valid(&self) -> bool {
		todo!(6);
	}

	// Valid bytess are represented by the characters A-Z or a-z
	pub fn is_valid_byte(byte: u8) -> bool {
		todo!(7);
	}
}
impl TryFrom<[u8; 4]> for ChunkType {
	type Error = Error;

	fn try_from(bytes: [u8; 4]) -> Result<Self> {
		todo!(2);
	}
}
impl fmt::Display for ChunkType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		todo!(8);
	}
}
impl FromStr for ChunkType {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		todo!(1);
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;
	use crate::chunk_type::ChunkType;

	#[test]
	pub fn test_chunk_type_from_bytes() {
		let expected: [u8; 4] = [82, 117, 83, 116];
		let actual: ChunkType = ChunkType::try_from([82, 117, 83, 116]).unwrap();
		assert_eq!(expected, actual.bytes());
	}
	#[test]
	pub fn test_chunk_type_from_str() {
		let expected: ChunkType = ChunkType::try_from([82, 117, 83, 116]).unwrap();
		let actual: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert_eq!(expected, actual);
	}

	#[test]
	pub fn test_chunk_type_is_critical() {
		let chunk: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_critical());
	}
	#[test]
	pub fn test_chunk_type_is_not_critical() {
		let chunk: ChunkType = ChunkType::from_str("ruSt").unwrap();
		assert!(!chunk.is_critical());
	}

	#[test]
	pub fn test_chunk_type_is_public() {
		let chunk: ChunkType = ChunkType::from_str("RUSt").unwrap();
		assert!(chunk.is_public());
	}
	#[test]
	pub fn test_chunk_type_is_not_public() {
		let chunk: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(!chunk.is_public());
	}

	#[test]
	pub fn test_chunk_type_is_reserved_bit_valid() {
		let chunk: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_reserved_bit_valid());
	}
	#[test]
	pub fn test_chunk_type_is_reserved_bit_invalid() {
		let chunk: ChunkType = ChunkType::from_str("Rust").unwrap();
		assert!(!chunk.is_reserved_bit_valid());
	}

	#[test]
	pub fn test_chunk_type_is_safe_to_copy() {
		let chunk: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_safe_to_copy());
	}
	#[test]
	pub fn test_chunk_type_is_unsafe_to_copy() {
		let chunk: ChunkType = ChunkType::from_str("RuST").unwrap();
		assert!(!chunk.is_safe_to_copy());
	}

	#[test]
	pub fn test_valid_chunk_is_valid() {
		let chunk: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk.is_valid());
	}
	#[test]
	pub fn test_invalid_chunk_is_valid() {
		let chunk: ChunkType = ChunkType::from_str("Rust").unwrap();
		assert!(!chunk.is_valid());

		let chunk: ChunkType = ChunkType::from_str("Rult").unwrap();
		assert!(!chunk.is_valid());
	}

	#[test]
	pub fn test_chunk_type_string() {
		let chunk: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert_eq!(&chunk.to_string(), "RuSt");
	}
	#[test]
	pub fn test_chunk_type_trait_impls() {
		let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
		let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
		let _chunk_string = format!("{}", chunk_type_1);
		let _are_chunks_equal = chunk_type_1 == chunk_type_2;
		assert!(_are_chunks_equal);
	}
}