use std::{
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use log::error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug)]
pub struct ConfigManager<S> {
    settings_file_path: PathBuf,
    settings: S,
}

impl<S> ConfigManager<S> {
    pub fn new(
        qualifier: &str,
        organization: &str,
        application: &str,
    ) -> anyhow::Result<ConfigManager<S>>
    where
        S: Default + DeserializeOwned + Serialize,
    {
        let default_config_dir_path = ProjectDirs::from(qualifier, organization, application)
            .unwrap()
            .config_dir()
            .to_path_buf();

        if !default_config_dir_path.exists() {
            fs::create_dir_all(&default_config_dir_path)?;
        }

        let settings_file_path = default_config_dir_path.join(format!("{}.json", application));

        let settings = if !settings_file_path.exists() {
            
            S::default()
        } else {
            match deserialize(&settings_file_path) {
                Ok(settings) => settings,
                Err(e) => {
                    error!("can't deserialize settings {e}");
                    S::default()
                }
            }
        };

        Ok(ConfigManager {
            settings_file_path,
            settings,
        })
    }

    fn settings_file_path(&self, name: &str) -> &PathBuf {
        &self.settings_file_path
    }

    pub fn settings(&self) -> &S {
        &self.settings
    }

    pub fn update(&mut self, mut f: impl FnMut(&mut S))
    where
        S: Serialize,
    {
        f(&mut self.settings);

        if let Err(e) = serialize(&self.settings_file_path, &self.settings) {
            error!("{e}");
        }
    }

    pub fn reload(&mut self) -> anyhow::Result<()>
    where
        S: DeserializeOwned,
    {
        self.settings = deserialize(&self.settings_file_path)?;
        Ok(())
    }
}

fn deserialize<T: DeserializeOwned>(path: &Path) -> anyhow::Result<T> {
    let str = fs::read_to_string(path)?;
    let t = json::from_str(&str)?;
    Ok(t)
}

fn serialize<T: Serialize>(path: &Path, rust_struct: &T) -> anyhow::Result<()> {
    let str = json::to_string_pretty(rust_struct)?;
    fs::write(path, str)?;
    Ok(())
}
