use std::path::Path;
//use std::collections::HashMap;
use crate::formats::wad::Wad;
use crate::errors::*;

/// Contains handles to every relevant game file.
pub struct GameFiles {
    pub dr2_data: Wad,
    pub dr2_data_us: Wad,
}

impl GameFiles {
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(dr2_data_path: P1, dr2_data_us_path: P2) -> Result<Self> {
        Ok(GameFiles {
            dr2_data: Wad::open(dr2_data_path)?,
            dr2_data_us: Wad::open(dr2_data_us_path)?,
        })
    }
}

pub mod music;
pub mod dialogue;
pub mod report_card;

pub fn extract<P: AsRef<Path>>(files: &GameFiles, path: P) -> Result<()> {
    let path = path.as_ref();
    std::fs::create_dir_all(&path)?;

    report_card::extract(files, path)?;
    dialogue::extract(files, path)?;
    music::extract(files, path)?;

    Ok(())
}

pub fn inject<P: AsRef<Path>>(files: &mut GameFiles, path: P) -> Result<()> {
    let path = path.as_ref();
    let _files = &files;

    dialogue::inject(files, &path.join("dialogue"))?;
    //music::inject(files, &path.join("music"))?;

    Ok(())
}
