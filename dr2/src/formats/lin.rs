use crate::errors::*;
use crate::formats::pak::Pak;

mod instructions;
use instructions::*;

#[derive(Debug)]
pub struct Lin {
    pub instructions: Vec<Instr>,
    pub strings: Option<Vec<String>>,
}

impl Lin {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let pak = Pak::from_bytes(data)?;
        let _instructions = &pak.entries[0];
        let strings = match pak.entries.get(1) {
            Some(strings) => {
                let strings_pak = Pak::from_bytes(&strings)?;
                Some(strings_pak.entries.iter()
                    .map(|entry| crate::decode_utf16(&entry))
                    .collect::<Result<Vec<_>>>()?
                )
            },
            None => None,
        };

        Ok(Lin {
            instructions: Vec::new(),
            strings,
        })
    }
}
