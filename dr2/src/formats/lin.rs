use std::fmt::Write;
use crate::errors::*;
use crate::formats::pak::Pak;

mod instructions;
use instructions::*;
pub mod script_parser;

#[derive(Debug)]
pub struct Lin {
    pub instructions: Vec<Instr>,
    pub strings: Option<Vec<String>>,
}

impl Lin {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let pak = Pak::from_bytes(data)?;
        let instructions = &pak.entries[0];
        let mut slice = instructions.as_ref();
        let mut instructions = Vec::new();
        loop {
            if slice.is_empty() {
                break;
            }

            let (instr, size) = Instr::read(slice)?;
            instructions.push(instr);
            slice = &slice[size..];
        }

        let strings = match pak.entries.get(1) {
            Some(strings) => {
                let strings_pak = Pak::from_bytes(&strings)?;
                Some(
                    strings_pak
                        .entries
                        .iter()
                        .map(|entry| crate::decode_utf16(&entry))
                        .collect::<Result<Vec<_>>>()?,
                )
            }
            None => None,
        };

        Ok(Lin {
            instructions,
            strings,
        })
    }

    /// Converts this .lin to the custom .script format.
    pub fn to_script(&self) -> Result<String> {
        let mut result = String::new();

        //writeln!(result, "#game dr2")?;

        for instr in &self.instructions {
            instr.write_as_script(&mut result, &self)?;
        }

        Ok(result)
    }
}

fn write_escaped<W: Write>(writer: &mut W, str: &str) -> Result<()> {
    writeln!(writer, "`")?;
    for c in str.chars() {
        match c {
            '\\' => write!(writer, "\\\\")?,
            '\t' => write!(writer, "\\t")?,
            '`' => write!(writer, "\\`")?,
            '\n' => write!(writer, "\n")?,
            '\x20'..='\x7e' => write!(writer, "{}", c)?,
            _ => write!(writer, "{}", c.escape_unicode())?,
        }
    }
    write!(writer, "`")?;

    Ok(())
}
