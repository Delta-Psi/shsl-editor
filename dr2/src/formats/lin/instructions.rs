use std::fmt::Write;
use crate::errors::*;
use byteorder::{ByteOrder, BE, LE};
use log::warn;

// from 0xa72980
const ARGUMENT_LENGTHS: &[u8] = &[
    0x02, 0x04, 0x02, 0x01, 0x04, 0x02, 0x08, 0x05, 0x05, 0x03, 0x03, 0x02, 0x02, 0x03, 0x02, 0x03,
    0x03, 0x04, 0x02, 0x02, 0x06, 0x04, 0x02, 0x0e, 0x0e, 0x05, 0x00, 0x05, 0x00, 0x00, 0x05, 0x07,
    0x05, 0x01, 0x03, 0x05, 0x04, 0x02, 0x03, 0x01, 0x04, 0x0d, 0x0c, 0x01, 0x02, 0x0c, 0x05, 0x02,
    0x02, 0x00, 0x01, 0x04, 0x03, 0x02, 0x01, 0x03, 0x02, 0x02, 0x04, 0x02, 0x04, 0x05, 0x02, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Debug)]
pub enum Instr {
    TextCount(u16),

    Format(u8),
    UI(u8, u8),
 
    Sprite(u8, u8, u8, u8, u8),
    Speaker(u8),
    Voice(u8, u8, u16, u8),
    Text(u16),

    //EndOfJump(u16),
    //StartOfJump(u16),

    //WaitForInput,
    //WaitFrame,

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
            0x08 => (Voice(data[2], data[3], BE::read_u16(&data[4..6]), data[6]), 7),

            0x1e => (Sprite(data[2], data[3], data[4], data[5], data[6]), 7),

            0x21 => (Speaker(data[2]), 3),
            0x25 => (UI(data[2], data[3]), 4),

            //0x2c => (EndOfJump(BE::read_u16(&data[2..4])), 4),
            //0x3b => (StartOfJump(BE::read_u16(&data[2..4])), 4),

            //0x4b => (WaitForInput, 2),
            //0x4c => (WaitFrame, 2),

            op @ 0x4e..=0xff => {
                warn!("invalid opcode {:x}", op);
                (Raw(op), 1)
            }

            op => {
                let arg_length = ARGUMENT_LENGTHS[op as usize] as usize;
                (
                    Unknown(op, data[2..2 + arg_length].to_owned()),
                    2 + arg_length,
                )
            }
        })
    }

    pub fn write_as_script<W: Write>(&self, writer: &mut W, lin: &super::Lin) -> Result<()> {
        use Instr::*;

        match self {
            TextCount(_count) => {
                writeln!(writer, "text_count auto")?;
            },

            Text(index) => {
                write!(writer, "text ")?;
                super::write_escaped(writer, &lin.strings.as_ref().unwrap()[*index as usize])?;
                writeln!(writer)?;
            },
            Speaker(character) => {
                writeln!(writer, "speaker {}", character)?;
            },
            Voice(character, chapter, line, volume) => {
                writeln!(writer, "voice {}, {}, {}, {}", character, chapter, line, volume)?;
            },
            Sprite(position, character, sprite, state, transition) => {
                writeln!(writer, "sprite {}, {}, {}, {}, {}",
                    position, character, sprite, state, transition)?;
            },
            
            Format(format) => {
                writeln!(writer, "format {}", format)?;
            },
            UI(element, state) => {
                write!(writer, "ui {}, ", element)?;
                match state {
                    0 => writeln!(writer, "disable")?,
                    1 => writeln!(writer, "enable")?,
                    _ => writeln!(writer, "{}", state)?,
                }
            },

            Unknown(opcode, args) => {
                write!(writer, "raw 0x70, 0x{:02x}", opcode)?;
                for byte in args.iter() {
                    write!(writer, ", 0x{:02x}", byte)?;
                }
                writeln!(writer)?;
            },
            Raw(byte) => {
                writeln!(writer, "raw 0x{:02x}", byte)?;
            }
        }

        Ok(())
    }
}
