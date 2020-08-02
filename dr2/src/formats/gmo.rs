use byteorder::{ByteOrder, LE};

pub mod file;
pub mod model;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    WrongMagicNumbers,
    IncompleteChunk,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            Io(error) => write!(f, "i/o error: {}", error),
            WrongMagicNumbers => write!(f, "wrong magic numbers"),
            IncompleteChunk => write!(f, "incomplete chunk"),
        }
    }
}
impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Gmo {
    pub file: file::File,
}

impl Gmo {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Error> {
        let magic_numbers = &data[0..16];
        if magic_numbers != b"OMG.00.1PSP\0\0\0\0\0" {
            return Err(Error::WrongMagicNumbers);
        }

        let data = &data[16..];
        let (file, size) = ChunkRef::read(data)?;
        if size != data.len() {
            panic!("trailing data");
        }

        Ok(Self {
            file: file::File::new(file),
        })
    }
}

#[derive(Debug)]
pub struct ChunkRef<'a> {
    pub type_: u16,
    pub header: &'a [u8],
    pub data: &'a [u8],
}

impl<'a> ChunkRef<'a> {
    pub fn read(data: &'a [u8]) -> Result<(Self, usize), Error> {
        if data.len() < 8 {
            return Err(Error::IncompleteChunk);
        }
        
        let type_ = LE::read_u16(&data[0..2]);
        let data_offset = match LE::read_u16(&data[2..4]) {
            0 => 8,
            offset => offset as usize,
        };
        let chunk_size = LE::read_u32(&data[4..8]) as usize;

        if data.len() < chunk_size {
            return Err(Error::IncompleteChunk);
        }

        let header = &data[8 .. data_offset];
        let data = &data[data_offset .. chunk_size];

        Ok((Self {
            type_,
            header: header,
            data: data,
        }, chunk_size))
    }

    pub fn children(&self) -> Chunks {
        Chunks {
            data: &self.data,
        }
    }
}

#[derive(Debug)]
pub struct Chunks<'a> {
    data: &'a [u8],
}

impl<'a> Iterator for Chunks<'a> {
    type Item = ChunkRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            None
        } else {
            let (chunk, size) = Self::Item::read(self.data).ok()?;
            self.data = &self.data[size..];
            Some(chunk)
        }
    }
}

pub trait Chunk {
    fn new(chunk: ChunkRef) -> Self;
}

#[derive(Debug)]
pub struct Generic {
    pub type_: u16,
    pub header: Vec<u8>,
    pub data: Vec<u8>,
}

impl Chunk for Generic {
    fn new(chunk: ChunkRef) -> Self {
        Self {
            type_: chunk.type_,
            header: chunk.header.to_owned(),
            data: chunk.data.to_owned(),
        }
    }
}
