use crate::errors::*;
use crate::formats::wad::Wad;
use crate::project::Project;
use serde::{Serialize, Deserialize};
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

#[derive(Clone, Copy, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub presents: bool,
    pub scripts: bool,
    pub dialogue: bool,
    pub music: bool,
    pub report_card: bool,
}

pub mod presents;
pub mod scripts;
pub mod dialogue;
pub mod music;
pub mod report_card;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    let config = project.config().game_data;

    if config.presents {
        presents::extract(project, files)?;
    }
    if config.scripts {
        scripts::extract(project, files)?;
    }
    if config.report_card {
        report_card::extract(project, files)?;
    }
    if config.dialogue {
        dialogue::extract(project, files)?;
    }
    if config.music {
        music::extract(project, files)?;
    }

    Ok(())
}

pub fn inject(project: &mut Project, files: &mut GameFiles) -> Result<()> {
    let config = project.config().game_data;

    if config.presents {
        presents::inject(project, files)?;
    }
    if config.scripts {
        scripts::inject(project, files)?;
    }
    if config.report_card {
        report_card::inject(project, files)?;
    }
    if config.dialogue {
        dialogue::inject(project, files)?;
    }
    if config.music {
        music::inject(project, files)?;
    }

    Ok(())
}
