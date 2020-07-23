use std::io::prelude::*;
use std::io::{BufReader, Cursor};
use std::borrow::Cow;
use byteorder::{ReadBytesExt, WriteBytesExt, LE};

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

pub struct Pak<'a> {
    pub entries: Box<[Cow<'a, [u8]>]>,
}

impl<'a> Pak<'a> {
    pub fn from_bytes(data: &'a [u8]) -> Result<Self> {
        let header = Header::read_from(Cursor::new(data))?;
        // check if no offset is larger than the actual data length
        if header.offsets.iter().any(|o| *o as usize >= data.len()) {
            bail!(ErrorKind::InvalidPakOffset);
        }

        let count = header.offsets.len();
        let mut entries = Vec::with_capacity(count);

        let mut offsets = header.offsets;
        offsets.push(data.len() as u32);

        for i in 0..count {
            let begin = offsets[i] as usize;
            let end = offsets[i+1] as usize;

            entries.push(data[begin..end].into());
        }

        Ok(Pak {
            entries: entries.into_boxed_slice(),
        })
    }

    pub fn repack(&self) -> Result<Vec<u8>> {
        let mut buf = std::io::Cursor::new(Vec::new());

        buf.write_u32::<LE>(self.entries.len() as u32)?;
        let mut offset: usize = 4 + self.entries.len()*4;
        for entry in self.entries.iter() {
            buf.write_u32::<LE>(offset as u32)?;
            offset += entry.len();
        }

        for entry in self.entries.iter() {
            buf.write(&entry)?;
        }

        Ok(buf.into_inner())
    }
}
