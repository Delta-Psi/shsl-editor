use crate::pak::Pak;
use crate::encoding::decode_utf16_le;

#[derive(Debug)]
pub struct Script {
    pub strings: Vec<String>,
}

impl Script {
    pub fn decode(data: &[u8]) -> Option<Self> {
        let script_pak = Pak::decode(data)?;

        match script_pak.entries.len() {
            1 | 2 => (),
            _ => return None,
        }

        // TODO: read instruction data

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
            strings
        })
    }
}
