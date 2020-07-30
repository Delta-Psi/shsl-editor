use crate::errors::*;
use error_chain::bail;
use byteorder::{WriteBytesExt, ByteOrder, BE, LE};
use log::warn;

// from 0xa72980
const ARGUMENT_LENGTHS: &[u8] = &[
    0x02, 0x04, 0x02, 0x01, 0x04, 0x02, 0x08, 0x05, 0x05, 0x03, 0x03, 0x02, 0x02, 0x03, 0x02, 0x03,
    0x03, 0x04, 0x02, 0x02, 0x06, 0x04, 0x02, 0x0e, 0x0e, 0x05, 0x00, 0x05, 0x00, 0x00, 0x05, 0x07,
    0x05, 0x01, 0x03, 0x05, 0x04, 0x02, 0x03, 0x01, 0x04, 0x0d, 0x0c, 0x01, 0x02, 0x0c, 0x05, 0x02,
    0x02, 0x00, 0x01, 0x04, 0x03, 0x02, 0x01, 0x03, 0x02, 0x02, 0x04, 0x02, 0x04, 0x05, 0x02, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Debug, PartialEq)]
pub enum Instr {
    TextCount(Option<u16>),

    Format(u8),
    UI(u8, u8),
 
    Sprite(u8, u8, u8, u8, u8),
    Speaker(u8),
    Voice(u8, u8, u16, u8),
    Text(u16),

    //EndOfJump(u16),
    //StartOfJump(u16),

    Pause,
    //WaitFrame,

    Raw(Vec<u8>),
}

impl Instr {
    // (instruction, bytes to advance)
    pub fn read(data: &[u8]) -> Result<(Self, usize)> {
        use Instr::*;

        // TODO: port this over from disassembly (0x57bd70/0x57bda0)
        // probably not just that function
        if data[0] != 0x70 {
            warn!("unknown byte {}", data[0]);
            return Ok((Raw(vec![data[0]]), 1));
        }

        Ok(match data[1] {
            0x00 => (TextCount(Some(LE::read_u16(&data[2..4]))), 4),
            0x02 => (Text(BE::read_u16(&data[2..4])), 4),
            0x03 => (Format(data[2]), 3),
            0x08 => (Voice(data[2], data[3], BE::read_u16(&data[4..6]), data[6]), 7),

            0x1e => (Sprite(data[2], data[3], data[4], data[5], data[6]), 7),

            0x21 => (Speaker(data[2]), 3),
            0x25 => (UI(data[2], data[3]), 4),

            //0x2c => (EndOfJump(BE::read_u16(&data[2..4])), 4),
            //0x3b => (StartOfJump(BE::read_u16(&data[2..4])), 4),

            0x4b => (Pause, 2),
            //0x4c => (WaitFrame, 2),

            op @ 0x4e..=0xff => {
                warn!("invalid opcode {:x}", op);
                (Raw(vec![op]), 1)
            }

            op => {
                let arg_length = ARGUMENT_LENGTHS[op as usize] as usize;
                (
                    Raw(data[0..2 + arg_length].to_owned()),
                    2 + arg_length,
                )
            }
        })
    }

    pub fn write_as_script<W: std::fmt::Write>(&self, writer: &mut W, lin: &super::Lin) -> Result<()> {
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

            Pause => {
                writeln!(writer, "pause")?;
            },

            Raw(bytes) => {
                write!(writer, "raw")?;

                for (i, byte) in bytes.iter().enumerate() {
                    if i > 0 {
                        write!(writer, ",")?;
                    }
                    write!(writer, " 0x{:02x}", byte)?;
                }
                writeln!(writer)?;
            }
        }

        Ok(())
    }

    pub fn from_script(instr: &super::script_parser::Instr, strings: &mut Vec<String>) -> Result<Self> {
        use self::Instr as I;
        use super::script_parser::*;
        use std::convert::TryInto;

        match instr.operation.1 {
            "text_count" => {
                if instr.args.len() != 1 {
                    bail!("text_count takes 1 argument");
                }

                Ok(I::TextCount(
                        match instr.args[0] {
                            Arg::Ident(Ident(_, "auto")) => None,
                            Arg::Int(Int(_, count)) => Some(count
                                .try_into()
                                .chain_err(|| "text count not in range")?),

                            _ => bail!("invalid argument"),
                        },
                ))
            },

            "text" => {
                if instr.args.len() != 1 {
                    bail!("text takes 1 argument");
                }

                if let Some(text) = instr.args[0].as_text() {
                    let index = strings.len();
                    strings.push(super::unescape(text)?);
                    Ok(I::Text(index
                            .try_into()
                            .chain_err(|| "text count overflow")?))
                } else {
                    bail!("invalid argument");
                }
            },

            "speaker" => {
                if instr.args.len() != 1 {
                    bail!("speaker takes 1 argument");
                }

                if let Some(character) = instr.args[0].as_int() {
                    Ok(I::Speaker(character
                            .try_into()
                            .chain_err(|| "not in range")?))
                } else {
                    bail!("invalid argument");
                }
            },

            "voice" => {
                if instr.args.len() != 4 {
                    bail!("voice takes 4 arguments");
                }

                Ok(I::Voice(
                        instr.args[0].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        instr.args[1].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        instr.args[2].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        instr.args[3].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                ))
            },

            "sprite" => {
                if instr.args.len() != 5 {
                    bail!("sprite takes 5 arguments");
                }

                Ok(I::Sprite(
                        instr.args[0].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        instr.args[1].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        instr.args[2].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        instr.args[3].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        instr.args[4].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                ))
            },

            "format" => {
                if instr.args.len() != 1 {
                    bail!("format takes 1 argument");
                }

                Ok(I::Format(
                        instr.args[0].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                ))
            },

            "ui" => {
                if instr.args.len() != 2 {
                    bail!("ui takes 2 arguments");
                }

                Ok(I::UI(
                        instr.args[0].as_int()
                            .ok_or("invalid argument")?
                            .try_into().chain_err(|| "not in range")?,
                        match instr.args[1] {
                            Arg::Ident(Ident(_, "disable")) => 0,
                            Arg::Ident(Ident(_, "enable")) => 1,
                            Arg::Int(Int(_, state)) => state
                                .try_into().chain_err(|| "not in range")?,
                            _ => bail!("invalid argument"),
                        },
                ))
            },

            "pause" => Ok(I::Pause),

            "raw" => {
                let mut bytes = Vec::with_capacity(instr.args.len());
                for arg in instr.args.iter() {
                    bytes.push(arg.as_int()
                        .ok_or("invalid argument")?
                        .try_into().chain_err(|| "not in range")?);
                }

                Ok(I::Raw(bytes))
            },

            _ => bail!("unknown operation: {}", instr.operation.1),
        }
    }

    pub fn encode<W: std::io::Write>(&self, writer: &mut W, lin: &super::Lin) -> std::io::Result<()> {
        use Instr::*;
        use std::convert::TryInto;

        match self {
            TextCount(count) => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x00)?;
                match count {
                    Some(count) => writer.write_u16::<LE>(*count)?,
                    None => match &lin.strings {
                        Some(strings) => writer.write_u16::<LE>(strings.len()
                            .try_into().unwrap())?,
                        None => writer.write_u16::<LE>(0)?,
                    },
                }
            },
            Text(index) => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x02)?;
                writer.write_u16::<BE>(*index)?;
            },
            Format(format) => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x03)?;
                writer.write_u8(*format)?;
            },
            Voice(character, chapter, line, volume) => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x08)?;
                writer.write_u8(*character)?;
                writer.write_u8(*chapter)?;
                writer.write_u16::<BE>(*line)?;
                writer.write_u8(*volume)?;
            },

            Sprite(position, character, sprite, state, transition) => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x1e)?;
                writer.write_u8(*position)?;
                writer.write_u8(*character)?;
                writer.write_u8(*sprite)?;
                writer.write_u8(*state)?;
                writer.write_u8(*transition)?;
            },

            Speaker(speaker) => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x21)?;
                writer.write_u8(*speaker)?;
            },
            UI(element, state) => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x25)?;
                writer.write_u8(*element)?;
                writer.write_u8(*state)?;
            },

            Pause => {
                writer.write_u8(0x70)?;
                writer.write_u8(0x4b)?;
            },

            Raw(bytes) => {
                writer.write_all(bytes)?;
            },
        }

        Ok(())
    }
}
