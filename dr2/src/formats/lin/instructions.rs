use crate::errors::*;
use byteorder::{ByteOrder, LE};

#[derive(Debug)]
pub enum Instr {
    TextCount(u16),

    Unknown(u8, Vec<u8>),
}

impl Instr {
    // (instruction, bytes to advance)
    pub fn read(data: &[u8]) -> Result<Option<(Self, usize)>> {
        use Instr::*;

        // TODO: port this over from disassembly (0x57bd70/0x57bda0)
        if data[0] != 0x70 {
            return Ok(None);
        }

        Ok(Some( match data[1] {
            0x00 => (TextCount(LE::read_u16(&data[2..4])), 4),

            _ => {
                match data[2..].iter().enumerate().find(|(_, v)| **v == 0x70) {
                    None => (Unknown(data[1], data[2..].to_owned()), data.len()),
                    Some((i, _)) => (Unknown(data[1], data[2..i+2].to_owned()), i+2),
                }
            }
        }))
    }
}
