use crate::errors::*;
use error_chain::bail;
use log::info;
use relative_path::{RelativePath, RelativePathBuf};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub game_data: crate::game_data::Config,
}

pub struct Project {
    base_path: PathBuf,
    config: ProjectConfig,
    sync_path: PathBuf,
    sync: HashMap<RelativePathBuf, std::time::SystemTime>,
}

impl Project {
    pub fn create<P: AsRef<Path>>(base_path: P, config: ProjectConfig) -> Result<Self> {
        let base_path = base_path.as_ref();
        if base_path.exists() {
            bail!("project path already exists");
        }

        std::fs::create_dir_all(&base_path)
            .chain_err(|| "could not create project directory structure")?;

        let config_path = base_path.join("Project.toml");
        info!("writing configuration to {}", config_path.display());
        std::fs::write(config_path,
            &toml::to_string_pretty(&config)
                .chain_err(|| "could not serialize given configuration")?.as_bytes())
            .chain_err(|| "could not write configuration file")?;

        let sync_path = base_path.join("Sync.toml");
        info!("writing blank sync file to {}", sync_path.display());
        std::fs::write(&sync_path, b"")
            .chain_err(|| "could not write sync file")?;

        let gitignore_path = base_path.join(".gitignore");
        info!("writing gitignore to {}", gitignore_path.display());
        std::fs::write(gitignore_path, b"Sync.toml\n")
            .chain_err(|| "coult not write gitignore")?;

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
        let config_data = std::fs::read(&config_path)
            .chain_err(|| "could not read configuration file")?;
        let config = toml::from_slice(&config_data)
            .chain_err(|| "could not deserialize configuration file")?;

        let sync_path = base_path.join("Sync.toml");
        info!("reading sync data from {}", sync_path.display());
        let sync_data = std::fs::read(&sync_path)
            .chain_err(|| "could not read sync file")?;
        let sync = toml::from_slice(&sync_data)
            .chain_err(|| "could not deserialize sync file")?;

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
            std::fs::create_dir_all(parent)
                .chain_err(|| format!("could not create directory structure for {}", path))?;
        }

        info!("writing {}", full_path.display());
        std::fs::write(&full_path, data)
            .chain_err(|| format!("could not write {}", path))?;

        self.sync.insert(
            path.to_owned(),
            std::fs::metadata(&full_path)
            .chain_err(|| format!("could not check metadata for {}", path))?
            .modified()
            .chain_err(|| format!("could not check modification time for {}", path))?);
        self.update_sync_file()
    }

    pub fn write_toml<P: AsRef<RelativePath>, T: Serialize>(
        &mut self,
        path: P,
        data: &T,
    ) -> Result<()> {
        let string = toml::to_string_pretty(data)
            .chain_err(|| format!("could not serialize {}", path.as_ref()))?;
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
        let modified = std::fs::metadata(&full_path)
            .chain_err(|| format!("could not check metadata for {}", path))?
            .modified()
            .chain_err(|| format!("could not check modification time for {}", path))?;

        if let Some(prev) = self.sync.get(path) {
            if *prev >= modified {
                return Ok(());
            }
        }

        // create/update entry
        info!("reading {}", full_path.display());
        let data = std::fs::read(&full_path)
            .chain_err(|| format!("could not open {}", path))?;
        func(&data)?;

        self.sync.insert(path.to_owned(), modified);
        self.update_sync_file()?;

        Ok(())
    }

    fn update_sync_file(&self) -> Result<()> {
        let string = toml::to_string_pretty(&self.sync)
            .chain_err(|| "could not serialize updated sync file")?;
        std::fs::write(&self.sync_path, &string.as_bytes())
            .chain_err(|| "could not update sync file")?;

        Ok(())
    }
}

