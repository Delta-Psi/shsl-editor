use std::path::Path;
//use std::collections::HashMap;
use crate::formats::wad::Wad;
use crate::errors::*;

pub struct WadPlus {
    pub wad: Wad,
    //paks: HashMap<String, Pak>,
}

impl WadPlus {
    pub fn new(wad: Wad) -> Self {
        Self {
            wad,
            //paks: HashMap::new(),
        }
    }

/*
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
*/
}

/// Contains handles to every relevant game file.
pub struct GameFiles {
    pub dr2_data: WadPlus,
    pub dr2_data_us: WadPlus,
}

impl GameFiles {
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(dr2_data_path: P1, dr2_data_us_path: P2) -> Result<Self> {
        Ok(GameFiles {
            dr2_data: WadPlus::new(Wad::open(dr2_data_path)?),
            dr2_data_us: WadPlus::new(Wad::open(dr2_data_us_path)?),
        })
    }
}

/*
pub trait Data: Sized {
    fn extract<P: AsRef<Path>>(files: &GameFiles, path: P) -> Result<()>;
    fn inject<P: AsRef<Path>>(files: &mut GameFiles, path: P) -> Result<()>;
    //fn load(path: &Path) -> Result<Self>;
}
*/

/*
/// References a WAD file.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum WadRef {
    Data,
    DataUs,
}
*/

pub mod music;
pub mod dialogue;

pub fn extract<P: AsRef<Path>>(files: &GameFiles, path: P) -> Result<()> {
    let path = path.as_ref();

    dialogue::extract(files, &path.join("dialogue"))?;

    Ok(())
}

pub fn inject<P: AsRef<Path>>(files: &mut GameFiles, path: P) -> Result<()> {
    let path = path.as_ref();

    dialogue::inject(files, &path.join("dialogue"))?;

    Ok(())
}
