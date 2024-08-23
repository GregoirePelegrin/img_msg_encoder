use std::error::Error;
use std::fmt;
use std::str;
use std::str::FromStr;

/// A validated PNG chunk type. See PNG spec for more details.
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChunkType {
	chunk_type: [u8; 4],
}
impl ChunkType {
	// Returns the raw bytes contained in this chunk type
	pub fn bytes(&self) -> [u8; 4] {
		self.chunk_type
	}

	// Returns the property state of the first byte as described in the PNG spec
	pub fn is_critical(&self) -> bool {
		self.chunk_type[0].is_ascii_uppercase()
	}

	// Returns the property state of the second byte as described in the PNG spec
	pub fn is_public(&self) -> bool {
		self.chunk_type[1].is_ascii_uppercase()
	}

	// Returns the property state of the third byte as described in the PNG spec
	pub fn is_reserved_bit_valid(&self) -> bool {
		self.chunk_type[2].is_ascii_uppercase()
	}

	// Returns the property state of the fourth byte as described in the PNG spec
	pub fn is_safe_to_copy(&self) -> bool {
		self.chunk_type[3].is_ascii_lowercase()
	}

	// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z
	// Note that this chunk type should always be valid as it is validated during construction
	pub fn is_valid(&self) -> bool {
		self.is_reserved_bit_valid()
			&& self.chunk_type.iter().fold(true, |acc: bool, byte: &u8| {acc && byte.is_ascii_alphabetic()})
	}

	// Valid bytes are represented by the characters A-Z or a-z
	// pub fn is_valid_byte(byte: u8) -> bool {
	// 	byte.is_ascii()
	// }
}
impl fmt::Display for ChunkType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let formatted_element = self.chunk_type
                        .iter()
                        .map(|elt| format!("{}", *elt as char))
			.collect::<Vec<_>>();
		write!(f, "{}", formatted_element.join(""))
	}
}
impl TryFrom<[u8; 4]> for ChunkType {
	type Error = Box<dyn Error>;

	fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
		let result: Self = Self{chunk_type: bytes};
		if bytes.iter().fold(true, |acc: bool, byte: &u8| {acc && byte.is_ascii_alphabetic()}) {
			return Ok(result);
		}
		Err("Invalid chunk type".into())
	}
}
impl FromStr for ChunkType {
	type Err = Box<dyn Error>;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let bytes: [u8; 4] = s.as_bytes().try_into()?;
		ChunkType::try_from(bytes)
	}
}

#[cfg(test)]
mod tests {
	use std::error::Error;
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
		let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk_type.is_critical());
	}
	#[test]
	pub fn test_chunk_type_is_not_critical() {
		let chunk_type: ChunkType = ChunkType::from_str("ruSt").unwrap();
		assert!(!chunk_type.is_critical());
	}

	#[test]
	pub fn test_chunk_type_is_public() {
		let chunk_type: ChunkType = ChunkType::from_str("RUSt").unwrap();
		assert!(chunk_type.is_public());
	}
	#[test]
	pub fn test_chunk_type_is_not_public() {
		let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(!chunk_type.is_public());
	}

	#[test]
	pub fn test_chunk_type_is_reserved_bit_valid() {
		let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk_type.is_reserved_bit_valid());
	}
	#[test]
	pub fn test_chunk_type_is_reserved_bit_invalid() {
		let chunk_type: ChunkType = ChunkType::from_str("Rust").unwrap();
		assert!(!chunk_type.is_reserved_bit_valid());
	}

	#[test]
	pub fn test_chunk_type_is_safe_to_copy() {
		let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk_type.is_safe_to_copy());
	}
	#[test]
	pub fn test_chunk_type_is_unsafe_to_copy() {
		let chunk_type: ChunkType = ChunkType::from_str("RuST").unwrap();
		assert!(!chunk_type.is_safe_to_copy());
	}

	#[test]
	pub fn test_valid_chunk_is_valid() {
		let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert!(chunk_type.is_valid());
	}
	#[test]
	pub fn test_invalid_chunk_is_valid() {
		let chunk_type: ChunkType = ChunkType::from_str("Rust").unwrap();
		assert!(!chunk_type.is_valid());

		let chunk_type: Result<ChunkType, Box<dyn Error>> = ChunkType::from_str("Ru1t");
		assert!(chunk_type.is_err());
	}

	#[test]
	pub fn test_chunk_type_string() {
		let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
		assert_eq!(&chunk_type.to_string(), "RuSt");
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
