use crate::formats::{wad::Wad, pak::Pak};
use std::path::Path;
use std::collections::HashMap;

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
    }

    links {
        Wad(crate::formats::wad::Error, crate::formats::wad::ErrorKind);
        Pak(crate::formats::pak::Error, crate::formats::pak::ErrorKind);
    }
}

pub struct WadPlus {
    pub wad: Wad,
    paks: HashMap<String, Pak>,
}

impl WadPlus {
    /// Allows easier manipulation of PAK files.
    pub fn get_pak(&mut self, path: &str) -> Result<&mut Pak> {
        if !self.paks.contains_key(path) {
            let mut buf = Vec::new();
            self.wad.read_file(path, &mut buf)?;
            let pak = Pak::from_bytes(&buf)?;
            self.paks.insert(path.to_string(), pak);
        }

        Ok(self.paks.get_mut(path).unwrap())
    }

    /// Ensures the data modified from `get_pak` is applied to the inner WAD.
    pub fn inject_paks(&mut self) -> Result<()> {
        use std::io::Cursor;

        for (path, pak) in &self.paks {
            let mut buf = Cursor::new(Vec::new());
            pak.encode(&mut buf)?;

            self.wad.inject_file(path, &buf.into_inner())?;
        }

        Ok(())
    }
}

/// Contains handles to every relevant game file.
pub struct GameFiles {
    pub dr2_data: WadPlus,
    pub dr2_data_us: WadPlus,
}

pub trait Data: Sized {
    fn extract(files: &GameFiles, path: &Path) -> Result<Self>;
    //fn inject(&self, files: &mut GameFiles, path: &Path) -> Result<()>;
}

pub mod music;
pub mod sprites;

pub struct GameData {
}
