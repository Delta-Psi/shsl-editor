use std::io::prelude::*;
use std::io::{BufReader, Cursor};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian as LE};

use error_chain::{error_chain, bail};
error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        InvalidOffset
        InvalidIndices
    }
}

pub struct Header {
    pub offsets: Vec<u32>,
}

impl Header {
    pub fn read_from<R: Read>(reader: R) -> Result<Self> {
        // buffer i/o calls
        let mut reader = BufReader::new(reader);

        // read entry count
        let count = reader.read_u32::<LE>()?;
        let mut offsets = Vec::with_capacity(count as usize);
        for i in 0..count as usize {
            let offset = reader.read_u32::<LE>()?;
            offsets.push(offset);

            // ensure offsets are ascending
            if i > 0 && offsets[i-1] >= offset {
                bail!(ErrorKind::InvalidOffset);
            }
        }

        Ok(Self {
            offsets,
        })
    }
}

pub enum Entry {
    Data(Box<[u8]>),
    /// Zero-terminated UTF-8/ASCII string.
    String(Box<str>),
    /// Zero-terminated LE UTF-16 string with BOM.
    WString(Box<str>),
    /// Nested PAK.
    Pak(Box<Pak>),
}

impl Entry {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut writer = Cursor::new(Vec::new());

        match self {
            Entry::Data(data) => writer.write_all(&data)?,
            Entry::String(string) => {
                string.bytes().try_for_each(|v| writer.write_u8(v))?;
                writer.write_u8(0)?;
            },
            Entry::WString(string) => {
                // BOM
                writer.write_all(b"\xff\xfe")?;
                string.encode_utf16().try_for_each(|v| writer.write_u16::<LE>(v))?;
                writer.write_u16::<LE>(0)?;
            },
            Entry::Pak(pak) => {
                pak.encode(&mut writer)?;
            },
        }

        Ok(writer.into_inner())
    }
}

pub struct Pak {
    entries: Vec<Entry>,
}

impl Pak {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let header = Header::read_from(Cursor::new(data))?;
        // check if no offset is larger than the actual data length
        if header.offsets.iter().any(|o| *o as usize >= data.len()) {
            bail!(ErrorKind::InvalidOffset);
        }

        let mut offsets = header.offsets.clone();
        offsets.push(data.len() as u32);
        
        let mut entries = Vec::with_capacity(header.offsets.len());

        for i in 0..header.offsets.len() {
            let begin = offsets[i] as usize;
            let end = offsets[i+1] as usize;

            entries.push(Entry::Data(data[begin..end].into()));
        }

        Ok(Pak {
            entries,
        })
    }

    /// Encodes the PAK and appends the resulting data to `buf`.
    pub fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u32::<LE>(self.entries.len() as u32)?;
        let entries = self.entries.iter().map(|e| e.encode()).collect::<Result<Vec<_>>>()?;

        // write offsets
        let mut offset = 4;
        for entry in &entries {
            writer.write_u32::<LE>(offset)?;
            offset += entry.len() as u32;
        }

        // write data
        for entry in &entries {
            writer.write_all(&entry)?;
        }

        Ok(())
    }

    pub fn index_mut(&mut self, indices: &[usize]) -> Result<&mut Entry> {
        if indices.is_empty() {
            bail!(ErrorKind::InvalidIndices);
        }

        let idx = indices[0];
        if idx >= self.entries.len() {
            bail!(ErrorKind::InvalidIndices);
        }

        if indices.len() == 1 {
            Ok(&mut self.entries[idx])
        } else {
            let entry = &mut self.entries[idx];
            if let Entry::Data(data) = entry {
                *entry = Entry::Pak(Box::new(Pak::from_bytes(&data)?));
            }

            if let Entry::Pak(pak) = entry {
                pak.index_mut(&indices[1..])
            } else {
                bail!(ErrorKind::InvalidIndices)
            }
        }
    }

    pub fn replace_data(&mut self, indices: &[usize], data: &[u8]) -> Result<()> {
        *self.index_mut(indices)? = Entry::Data(data.into());
        Ok(())
    }

    pub fn replace_string(&mut self, indices: &[usize], string: &str) -> Result<()> {
        *self.index_mut(indices)? = Entry::String(string.into());
        Ok(())
    }

    pub fn replace_wide_string(&mut self, indices: &[usize], string: &str) -> Result<()> {
        *self.index_mut(indices)? = Entry::WString(string.into());
        Ok(())
    }
}
