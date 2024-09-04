use crc::{Crc, CRC_32_ISO_HDLC};
use std::error::Error;
use std::fmt;
use crate::chunk_type::ChunkType;

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
impl Chunk {
    pub const DEFAULT_UPPERCASE: u8 = 90;
    pub const DEFAULT_LOWERCASE: u8 = 122;

    // Init functions
    pub fn try_from_type_data(chunk_type: [u8; 4], data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let bytes: Vec<u8> = chunk_type
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc: u32 = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&bytes);

        Ok(Chunk{
            length: data.len() as u32,
            chunk_type: ChunkType::try_from(chunk_type)?,
            data,
            crc
        })
    }
    pub fn try_from_bytes(bytes: &[u8], default: Option<bool>) -> Result<Self, Box<dyn Error>> {
        if bytes.len() < 4 + 4 + 4 {
            return Err(
                format!(
                    "Invalid chunk (chunk total size cannot be inferior to 12 bytes ({}))",
                    bytes.len()
                ).into()
            );
        }

        let length: u32 = u32::from_be_bytes(bytes[0..4].try_into()?);
        let given_chunk_type: [u8; 4] = bytes[4..8].try_into()?;
        let mut new_chunk_type: [u8; 4];
        match default {
            Some(true) => {
                new_chunk_type = given_chunk_type.map(
                    |char: u8| {
                        if char >= 97 {Self::DEFAULT_LOWERCASE} else {Self::DEFAULT_UPPERCASE}
                    }
                );
            }
            _ => {
                new_chunk_type = given_chunk_type;
            }
        }
        let data: Vec<u8> = bytes[8..(8 + length as usize)].to_vec();
        let computed_crc: u32 = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(
            &bytes[4..(4 + 4 + length as usize)]
        );
        let crc: u32 = u32::from_be_bytes(
            bytes[(4 + 4 + length as usize)..(4 + 4 + length as usize + 4)].try_into()?
        );
        if computed_crc != crc {
            return Err(format!(
                "Invalid chunk (given chunk crc isn't equal to computed crc: {} != {})",
                crc, computed_crc
            ).into());
        }

        Chunk::try_from_type_data(given_chunk_type, data)
    }

    // The length of the data portion of the chunk
    pub fn length(&self) -> u32 {
        self.length
    }

    // The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    // The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    // The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    // Returns the data stored in this chunk as a `String`. This function will return an error
    // if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String, Box<dyn Error>> {
        let result: String = String::from_utf8(self.data.clone())?;
        Ok(result)
    }

    // Returns this chunk as a byte sequences described by the PNG spec.
    // The following data is included in this byte sequence in order:
    // 1. Length of the data *(4 bytes)*
    // 2. Chunk type *(4 bytes)*
    // 3. The data itself *(`length` bytes)*
    // 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied().collect()
    }
}
impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn Error>;

    fn try_from(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Self::try_from_bytes(bytes, Some(true))
    }
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data length: {} bytes", self.data().len())?;
        writeln!(f, "  Content: {}", String::from_utf8_lossy(self.data()))?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::str::FromStr;

    use crate::chunk::Chunk;
    use crate::chunk_type::ChunkType;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type: &[u8] = "RuSt".as_bytes();
        let message_bytes: &[u8] = "This is where your secret message will be!".as_bytes();
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
    fn test_new_chunk() {
        let chunk_type: ChunkType = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk: Chunk = Chunk::try_from_type_data(
            <&[u8] as TryInto<[u8; 4]>>::try_into("RuSt".as_bytes()).unwrap(),
            data
        ).unwrap();

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }
    #[test]
    fn test_chunk_length() {
        let chunk: Chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }
    #[test]
    fn test_chunk_type() {
        let chunk: Chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }
    #[test]
    fn test_chunk_string() {
        let chunk: Chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string: String = String::from("This is where your secret message will be!");

        assert_eq!(chunk_string, expected_chunk_string);
    }
    #[test]
    fn test_chunk_crc() {
        let chunk: Chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type: &[u8] = "RuSt".as_bytes();
        let message_bytes: &[u8] = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string: String = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }
    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type: &[u8] = "RuSt".as_bytes();
        let message_bytes: &[u8] = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Result<Chunk, Box<dyn Error>> = Chunk::try_from(chunk_data.as_ref());
        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type: &[u8] = "RuSt".as_bytes();
        let message_bytes: &[u8] = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
