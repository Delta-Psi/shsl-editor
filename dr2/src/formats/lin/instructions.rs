use crate::errors::*;
use byteorder::{ByteOrder, LE, BE};
use log::warn;

// from 0xa72980
const ARGUMENT_LENGTHS: &[u8] = &[
    0x02,   0x04,   0x02,   0x01,   0x04,   0x02,   0x08,   0x05,
    0x05,   0x03,   0x03,   0x02,   0x02,   0x03,   0x02,   0x03,
    0x03,   0x04,   0x02,   0x02,   0x06,   0x04,   0x02,   0x0e,
    0x0e,   0x05,   0x00,   0x05,   0x00,   0x00,   0x05,   0x07,
    0x05,   0x01,   0x03,   0x05,   0x04,   0x02,   0x03,   0x01,
    0x04,   0x0d,   0x0c,   0x01,   0x02,   0x0c,   0x05,   0x02,
    0x02,   0x00,   0x01,   0x04,   0x03,   0x02,   0x01,   0x03,
    0x02,   0x02,   0x04,   0x02,   0x04,   0x05,   0x02,   0x00,
    0x00,   0x00,   0x00,   0x00,   0x00,   0x00,   0x00,   0x00,
    0x00,   0x00,   0x00,   0x00,   0x00,   0x00,
];

#[derive(Debug)]
pub enum Instr {
    TextCount(u16),
    Text(u16),
    Format(u8),

    EndOfJump(u16),
    StartOfJump(u16),

    WaitForInput,
    WaitFrame,

    Unknown(u8, Vec<u8>),
    Raw(u8),
}

impl Instr {
    // (instruction, bytes to advance)
    pub fn read(data: &[u8]) -> Result<(Self, usize)> {
        use Instr::*;

        // TODO: port this over from disassembly (0x57bd70/0x57bda0)
        // probably not just that function
        if data[0] != 0x70 {
            warn!("unknown byte {}", data[0]);
            return Ok((Raw(data[0]), 1));
        }

        Ok(match data[1] {
            0x00 => (TextCount(LE::read_u16(&data[2..4])), 4),
            0x02 => (Text(BE::read_u16(&data[2..4])), 4),
            0x03 => (Format(data[2]), 3),

            0x2c => (EndOfJump(BE::read_u16(&data[2..4])), 4),
            0x3b => (StartOfJump(BE::read_u16(&data[2..4])), 4),

            0x4b => (WaitForInput, 2),
            0x4c => (WaitFrame, 2),

            op @ 0x4e..=0xff => {
                warn!("invalid opcode {:x}", op);
                (Raw(op), 1)
            },

            op => {
                let arg_length = ARGUMENT_LENGTHS[op as usize] as usize;
                (Unknown(op, data[2..2+arg_length].to_owned()), 2+arg_length)
            }
        })
    }
}
