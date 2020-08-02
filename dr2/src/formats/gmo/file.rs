use super::*;

#[derive(Debug)]
pub struct File {
    pub header: Vec<u8>,
    pub subfiles: Vec<Subfile>,
}

impl Chunk for File {
    fn new(chunk: ChunkRef) -> Self {
        assert_eq!(chunk.type_, 0x0002, "chunk type doesn't match");

        Self {
            header: chunk.header.to_owned(),
            subfiles: chunk.children()
                .map(Subfile::new)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Subfile {
    pub header: Vec<u8>,
    pub chunks: Vec<SubfileChunk>,
}

impl Chunk for Subfile {
    fn new(chunk: ChunkRef) -> Self {
        assert_eq!(chunk.type_, 0x0003, "chunk type doesn't match");

        Self {
            header: chunk.header.to_owned(),
            chunks: chunk.children()
                .map(SubfileChunk::new)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub enum SubfileChunk {
    Model(model::Model),

    Generic(Generic),
}

impl Chunk for SubfileChunk {
    fn new(chunk: ChunkRef) -> Self {
        match chunk.type_ {
            // 0x0004: bone
            0x0005 => SubfileChunk::Model(model::Model::new(chunk)),

            _ => SubfileChunk::Generic(Generic::new(chunk)),
        }
    }
}
