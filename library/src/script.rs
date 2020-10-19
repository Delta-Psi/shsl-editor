use crate::pak::Pak;
use crate::encoding::decode_utf16_le;

mod instruction;
pub use instruction::{Instruction, Instructions};

#[derive(Debug)]
pub struct Script {
    pub instructions: Vec<Instruction>,
    pub strings: Vec<String>,
}

impl Script {
    pub fn decode(data: &[u8]) -> Option<Self> {
        let script_pak = Pak::decode(data)?;

        match script_pak.entries.len() {
            1 | 2 => (),
            _ => return None,
        }

        let instruction_data = script_pak.entries[0];
        let instructions = Instructions::decode(instruction_data).collect();

        let strings;
        if script_pak.entries.len() == 2 {
            // read string data
            let strings_pak = Pak::decode(script_pak.entries[1])?;
            strings = strings_pak.entries.into_iter()
                .map(|e| decode_utf16_le(e))
                .collect::<Option<_>>()?;
        } else {
            strings = Vec::new();
        }

        Some(Script {
            instructions,
            strings,
        })
    }

    pub fn decompile(&self) -> String {
        let mut result = String::new();

        for instruction in &self.instructions {
            result.push_str(&instruction.decompile(self));
            result.push('\n');
        }

        result
    }
}
