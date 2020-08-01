use crate::errors::*;
use crate::formats::wad::Wad;
use crate::project::Project;
use serde::{Serialize, Deserialize};
use std::path::Path;
use relative_path::RelativePath;
use error_chain::bail;

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

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub presents: bool,
    pub scripts: bool,
    pub dialogue: bool,
    pub music: bool,
    pub report_card: bool,

    pub extra: Vec<String>,
}

pub mod presents;
pub mod scripts;
pub mod dialogue;
pub mod music;
pub mod report_card;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    let config = project.config().game_data.clone();

    for extra_path in &config.extra {
        let split: Vec<&str> = extra_path.splitn(2, ':').collect();
        let wad_name = split[0];
        let wad_path = split[1];
        let path = RelativePath::new("extra").join(wad_name).join(wad_path);

        let wad = match wad_name {
            "dr2_data" => &files.dr2_data,
            "dr2_data_us" => &files.dr2_data_us,
            "dr2_data_keyboard" => &files.dr2_data_keyboard,
            "dr2_data_keyboard_us" => &files.dr2_data_keyboard_us,
            _ => bail!("unknown wad: {}", wad_name),
        };

        project.write_file(path, || wad.read_file(wad_path))?;
    }

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
    let config = project.config().game_data.clone();

    for extra_path in &config.extra {
        let split: Vec<&str> = extra_path.splitn(2, ':').collect();
        let wad_name = split[0];
        let wad_path = split[1];
        let path = RelativePath::new("extra").join(wad_name).join(wad_path);

        let wad = match wad_name {
            "dr2_data" => &mut files.dr2_data,
            "dr2_data_us" => &mut files.dr2_data_us,
            "dr2_data_keyboard" => &mut files.dr2_data_keyboard,
            "dr2_data_keyboard_us" => &mut files.dr2_data_keyboard_us,
            _ => bail!("unknown wad: {}", wad_name),
        };

        project.open_file(path, |data| wad.inject_file(wad_path, &data))?;
    }

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
