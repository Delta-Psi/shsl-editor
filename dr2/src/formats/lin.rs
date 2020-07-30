use std::fmt::Write;
use std::borrow::Cow;
use crate::errors::*;
use crate::formats::pak::Pak;
use error_chain::bail;

mod instructions;
use instructions::*;
pub mod script_parser;

#[derive(Debug, PartialEq)]
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

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        for instr in &self.instructions {
            instr.encode(&mut buf, &self)?;
        }

        let entries = match &self.strings {
            None => vec![Cow::Owned(buf)],
            Some(strings) => {
                let entries = strings.iter()
                    .map(|string| crate::encode_utf16(string)
                        .map(Cow::Owned))
                    .collect::<Result<Vec<_>>>()?;

                let strings = Pak {
                    entries,
                }.repack()?;

                vec![Cow::Owned(buf), Cow::Owned(strings)]
            },
        };

        Pak {
            entries,
        }.repack()
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

    pub fn from_script(input: &str) -> Result<Self> {
        let script = script_parser::parse_script(input)
            .chain_err(|| "could not parse script")?;
        let mut instructions = Vec::new();
        let mut strings = Vec::new();

        for instr in script.instrs {
            instructions.push(Instr::from_script(&instr, &mut strings)?);
        }

        Ok(Lin {
            instructions,
            strings: match strings.len() {
                0 => None,
                _ => Some(strings),
            },
        })
    }
}

fn write_escaped<W: Write>(writer: &mut W, str: &str) -> Result<()> {
    writeln!(writer, "`")?;
    for c in str.chars() {
        match c {
            '\\' => write!(writer, "\\\\")?,
            '\t' => write!(writer, "\\t")?,
            '`' => write!(writer, "\\`")?,
            '\n' => writeln!(writer)?,
            '\x20'..='\x7e' => write!(writer, "{}", c)?,
            _ => write!(writer, "{}", c.escape_unicode())?,
        }
    }
    write!(writer, "`")?;

    Ok(())
}

fn unescape(string: &str) -> Result<String> {
    let mut result = String::with_capacity(string.len());
    let mut escaped = false;
    for c in string.chars() {
        if escaped {
            match c {
                '\\' => result.push('\\'),
                't' => result.push('\t'),
                '`' => result.push('`'),
                _ => bail!("unknown escape sequence"),
            }

            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else {
            result.push(c);
        }
    }

    Ok(result)
}
