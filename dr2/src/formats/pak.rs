use std::io::prelude::*;
use std::io::{BufReader, Cursor};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian as LE};

use crate::errors::*;
use error_chain::bail;

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
                bail!(ErrorKind::InvalidPakOffset);
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
    pub fn as_string(&mut self) -> Option<String> {
        match self {
            Entry::Data(data) => {
                let (i, _) = data.iter().enumerate().find(|(_, v)| **v == 0)?;
                let string = String::from_utf8(data[0..i].to_owned()).ok()?;
                *self = Entry::String(string.clone().into_boxed_str());
                Some(string)
            },
            Entry::String(string) => Some(string.to_string()),
            Entry::WString(_) => None,
            Entry::Pak(_) => None,
        }
    }

    pub fn as_wstring(&mut self) -> Option<String> {
        match self {
            Entry::Data(data) => {
                use std::io::prelude::*;
                let mut cursor = std::io::Cursor::new(data.as_ref());

                let mut bom = [0, 0];
                cursor.read_exact(&mut bom).ok()?;
                if bom != [0xff, 0xfe] {
                    return None;
                }

                let mut data = Vec::with_capacity(data.len()/2 - 4);
                let mut v = cursor.read_u16::<LE>().ok()?;
                while v != 0 {
                    data.push(v);
                    v = cursor.read_u16::<LE>().ok()?;
                }

                let string = String::from_utf16(&data).ok()?;
                *self = Entry::WString(string.clone().into_boxed_str());
                Some(string)
            },
            Entry::String(_) => None,
            Entry::WString(string) => Some(string.to_string()),
            Entry::Pak(_) => None,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
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
    pub entries: Vec<Entry>,
}

impl Pak {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let header = Header::read_from(Cursor::new(data))?;
        // check if no offset is larger than the actual data length
        if header.offsets.iter().any(|o| *o as usize >= data.len()) {
            bail!(ErrorKind::InvalidPakOffset);
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

    pub fn index_pak(&mut self, indices: &[usize]) -> Result<&mut Pak> {
        if indices.is_empty() {
            return Ok(self);
        }

        let idx = indices[0];
        if idx >= self.entries.len() {
            bail!(ErrorKind::InvalidPakIndices);
        }

        let entry = &mut self.entries[idx];
        if let Entry::Data(data) = entry {
            *entry = Entry::Pak(Box::new(Pak::from_bytes(&data)?));
        }

        if let Entry::Pak(pak) = entry {
            pak.index_pak(&indices[1..])
        } else {
            bail!(ErrorKind::InvalidPakIndices)
        }
    }

    pub fn index(&mut self, indices: &[usize]) -> Result<&mut Entry> {
        Ok(&mut self.index_pak(&indices[0..indices.len()-1])?.entries[indices[indices.len()-1]])
    }

    pub fn replace_data(&mut self, indices: &[usize], data: &[u8]) -> Result<()> {
        *self.index(indices)? = Entry::Data(data.into());
        Ok(())
    }

    pub fn replace_string(&mut self, indices: &[usize], string: &str) -> Result<()> {
        *self.index(indices)? = Entry::String(string.into());
        Ok(())
    }

    pub fn replace_wide_string(&mut self, indices: &[usize], string: &str) -> Result<()> {
        *self.index(indices)? = Entry::WString(string.into());
        Ok(())
    }
}
