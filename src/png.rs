

#[cfg(test)]
mod tests {

    use crate::chunk_type::ChunkType;
    use crate::chunk::Chunk;
    use crate::png::Png;

    fn testing_chunks() -> Vec<Chunk> {
        vec![
            chunk_from_strings("FrSt", "I am the first chunk").unwrap(),
            chunk_from_strings("miDl", "I am another chunk").unwrap(),
            chunk_from_strings("LASt", "I am the last chunk").unwrap(),
        ]
    }

    fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
    }

    fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk> {
        use std::str::FromStr;

        let chunk_type: ChunkType = ChunkType::from_str(chunk_type)?;
        let data: Vec<u8> = data.bytes().collect();

        Ok(Chunk::new(chunk_type, data))
    }

    // TODO: Add tests
}