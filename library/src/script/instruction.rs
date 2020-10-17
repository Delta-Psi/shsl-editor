use byteorder::{ByteOrder, BE};

// from 0xa72980
const ARGUMENT_LENGTHS: &[u8] = &[
    0x02, 0x04, 0x02, 0x01, 0x04, 0x02, 0x08, 0x05,
    0x05, 0x03, 0x03, 0x02, 0x02, 0x03, 0x02, 0x03,
    0x03, 0x04, 0x02, 0x02, 0x06, 0x04, 0x02, 0x0e,
    0x0e, 0x05, 0x00, 0x05, 0x00, 0x00, 0x05, 0x07,
    0x05, 0x01, 0x03, 0x05, 0x04, 0x02, 0x03, 0x01,
    0x04, 0x0d, 0x0c, 0x01, 0x02, 0x0c, 0x05, 0x02,
    0x02, 0x00, 0x01, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x02, 0x02, 0x04, 0x02, 0x04, 0x05, 0x02, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Debug)]
pub enum Instruction {
    Text(u16),

    Raw(Vec<u8>),
}

impl Instruction {
    pub fn read(data: &[u8]) -> Option<(Self, usize)> {
        use Instruction::*;

        if *data.get(0)? != 0x70 {
            return Some((Raw(vec![data[0]]), 1));
        }

        Some(match data.get(1)? {
            0x02 => (Text(BE::read_u16(data.get(2..)?)), 4), 

            op @ 0x4e ..= 0xff => {
                eprintln!("warning: invalid opcode 0x{:x}", op);
                (Raw(data[0..2].to_owned()), 2)
            }

            op => {
                let arg_length = ARGUMENT_LENGTHS[*op as usize] as usize;
                (
                    Raw(data[0..2 + arg_length].to_owned()),
                    2 + arg_length,
                )
            }
        })
    }

    pub fn read_and_advance(data: &mut &[u8]) -> Option<Self> {
        let (instruction, size) = Self::read(*data)?;
        *data = &data[size..];
        Some(instruction)
    }

    pub fn decompile(&self, script: &super::Script) -> String {
        use Instruction::*;
        use std::fmt::Write;

        match self {
            Text(index) => {
                let string = &script.strings[*index as usize];

                format!("text `\n{}`", string)
            }

            Raw(data) => {
                let mut result = String::with_capacity(8 + 6*(data.len()-1));

                write!(&mut result, "raw 0x{:02x}", data[0]).unwrap();
                for i in 1..data.len() {
                    write!(&mut result, ", 0x{:02x}", data[i]).unwrap();
                }

                result
            }
        }
    }
}
