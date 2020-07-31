use crate::errors::*;
use crate::formats::wad::Wad;
use crate::project::Project;
use serde::Deserialize;
use std::path::Path;

/// Contains handles to every relevant game file.
pub struct GameFiles {
    pub dr2_data: Wad,
    pub dr2_data_us: Wad,
    pub dr2_data_keyboard: Wad,
    pub dr2_data_keyboard_us: Wad,
}

impl GameFiles {
    pub fn load<P: AsRef<Path>>(game_path: P) -> Result<Self> {
        let game_path = game_path.as_ref();

        Ok(GameFiles {
            dr2_data: Wad::open(&game_path.join("dr2_data.wad"))?,
            dr2_data_us: Wad::open(&game_path.join("dr2_data_us.wad"))?,
            dr2_data_keyboard: Wad::open(&game_path.join("dr2_data_keyboard.wad"))?,
            dr2_data_keyboard_us: Wad::open(&game_path.join("dr2_data_keyboard_us.wad"))?,
        })
    }
}

pub mod presents;
pub mod scripts;
pub mod dialogue;
pub mod music;
pub mod report_card;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    presents::extract(project, files)?;
    scripts::extract(project, files)?;
    report_card::extract(project, files)?;
    dialogue::extract(project, files)?;
    music::extract(project, files)?;

    Ok(())
}

pub fn inject(project: &mut Project, files: &mut GameFiles) -> Result<()> {
    presents::inject(project, files)?;
    scripts::inject(project, files)?;
    report_card::inject(project, files)?;
    dialogue::inject(project, files)?;
    music::inject(project, files)?;

    Ok(())
}
