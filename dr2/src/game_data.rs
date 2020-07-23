use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::formats::wad::Wad;
use crate::errors::*;
use error_chain::bail;
use relative_path::{RelativePath, RelativePathBuf};
use serde::{Serialize, Deserialize};
use log::info;

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

// TODO
#[derive(Default, Serialize, Deserialize)]
pub struct ProjectConfig {
}

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

        self.sync.insert(path.to_owned(), std::fs::metadata(&full_path)?.modified()?);
        self.update_sync_file()
    }

    pub fn write_toml<P: AsRef<RelativePath>, T: Serialize>(&mut self, path: P, data: &T) -> Result<()> {
        let string = toml::to_string_pretty(data)?;
        self.write_file(path, &string.as_bytes())
    }

    fn update_sync_file(&self) -> Result<()> {
        let string = toml::to_string_pretty(&self.sync)?;
        std::fs::write(&self.sync_path, &string.as_bytes())?;

        Ok(())
    }
}

pub mod music;
pub mod dialogue;
pub mod report_card;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    report_card::extract(project, files)?;
    dialogue::extract(project, files)?;
    music::extract(project, files)?;

    Ok(())
}

pub fn inject<P: AsRef<Path>>(files: &mut GameFiles, path: P) -> Result<()> {
    let path = path.as_ref();
    let _files = &files;

    dialogue::inject(files, &path.join("dialogue"))?;
    //music::inject(files, &path.join("music"))?;

    Ok(())
}
