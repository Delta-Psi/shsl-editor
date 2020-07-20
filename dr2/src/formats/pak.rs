use std::io::prelude::*;
use std::io::BufReader;
use byteorder::{ReadBytesExt, LittleEndian as LE};

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        InvalidOffset
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
        for _ in 0..count {
            let offset = reader.read_u32::<LE>()?;
            offsets.push(offset);
        }

        Ok(Self {
            offsets,
        })
    }
}
