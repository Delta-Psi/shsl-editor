use crate::errors::*;
use crate::formats::wad::Wad;
use error_chain::bail;
use log::info;
use relative_path::{RelativePath, RelativePathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

// TODO
#[derive(Default, Serialize, Deserialize)]
pub struct ProjectConfig {}

pub struct Project {
    base_path: PathBuf,
    config: ProjectConfig,
    sync_path: PathBuf,
    pub sync: HashMap<RelativePathBuf, std::time::SystemTime>,
}

impl Project {
    pub fn create<P: AsRef<Path>>(base_path: P, config: ProjectConfig) -> Result<Self> {
        let base_path = base_path.as_ref();
        if base_path.exists() {
            bail!("project path already exists");
        }

        std::fs::create_dir_all(&base_path)?;

        let config_path = base_path.join("Project.toml");
        info!("writing configuration to {}", config_path.display());
        std::fs::write(config_path, &toml::to_string_pretty(&config)?.as_bytes())?;

        let sync_path = base_path.join("Sync.toml");
        info!("writing blank sync file to {}", sync_path.display());
        std::fs::write(&sync_path, &[])?;

        let gitignore_path = base_path.join(".gitignore");
        info!("writing gitignore to {}", gitignore_path.display());
        std::fs::write(gitignore_path, b"Sync.toml\n")?;

        Ok(Self {
            base_path: base_path.to_path_buf(),
            config,
            sync_path,
            sync: HashMap::new(),
        })
    }

    pub fn open<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref();

        let config_path = base_path.join("Project.toml");
        info!("reading configuration from {}", config_path.display());
        let config_data = std::fs::read(&config_path)?;
        let config = toml::from_slice(&config_data)?;

        let sync_path = base_path.join("Sync.toml");
        info!("reading sync data from {}", sync_path.display());
        let sync_data = std::fs::read(&sync_path)?;
        let sync = toml::from_slice(&sync_data)?;

        Ok(Self {
            base_path: base_path.to_path_buf(),
            config,
            sync_path,
            sync,
        })
    }

    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    pub fn config(&self) -> &ProjectConfig {
        &self.config
    }

    pub fn write_file<P: AsRef<RelativePath>>(&mut self, path: P, data: &[u8]) -> Result<()> {
        let path = path.as_ref();
        let full_path = path.to_path(&self.base_path);
        // ensure the parent directory exists
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        info!("writing {}", full_path.display());
        std::fs::write(&full_path, data)?;

        self.sync
            .insert(path.to_owned(), std::fs::metadata(&full_path)?.modified()?);
        self.update_sync_file()
    }

    pub fn write_toml<P: AsRef<RelativePath>, T: Serialize>(
        &mut self,
        path: P,
        data: &T,
    ) -> Result<()> {
        let string = toml::to_string_pretty(data)?;
        self.write_file(path, &string.as_bytes())
    }

    /// Only executes the closure if the file has been edited.
    pub fn open_file<P: AsRef<RelativePath>, F: FnOnce(&[u8]) -> Result<()>>(
        &mut self,
        path: P,
        func: F,
    ) -> Result<()> {
        let path = path.as_ref();
        let full_path = path.to_path(&self.base_path);
        let modified = std::fs::metadata(&full_path)?.modified()?;

        if let Some(prev) = self.sync.get(path) {
            if *prev < modified {
                info!("reading {}", full_path.display());
                let data = std::fs::read(&full_path)?;
                func(&data)?;

                self.sync.insert(path.to_owned(), modified);
                self.update_sync_file()?;
            }
        } else {
            unreachable!("file to be opened not in sync file");
        }

        Ok(())
    }

    fn update_sync_file(&self) -> Result<()> {
        let string = toml::to_string_pretty(&self.sync)?;
        std::fs::write(&self.sync_path, &string.as_bytes())?;

        Ok(())
    }
}

pub mod scripts;
pub mod dialogue;
pub mod music;
pub mod report_card;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    scripts::extract(project, files)?;
    report_card::extract(project, files)?;
    dialogue::extract(project, files)?;
    music::extract(project, files)?;

    Ok(())
}

pub fn inject(project: &mut Project, files: &mut GameFiles) -> Result<()> {
    report_card::inject(project, files)?;
    dialogue::inject(project, files)?;
    music::inject(project, files)?;

    Ok(())
}
