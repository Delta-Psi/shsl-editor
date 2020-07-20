use std::io::prelude::*;
use std::io::BufReader;
use byteorder::{ReadBytesExt, LittleEndian as LE};

use error_chain::{error_chain, bail};
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
