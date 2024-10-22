use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};
use figment::{value::Dict, Figment, Metadata, Profile, Provider};
use serde::de::Error;

pub struct CosmicRonProvider {
    path: PathBuf,
}

impl CosmicRonProvider {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }
}

impl Provider for CosmicRonProvider {
    fn metadata(&self) -> figment::Metadata {
        Metadata::named("cosmic ron provider")
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        let map = self.data_impl().map_err(figment::Error::custom)?;

        Ok(map)
    }
}

impl CosmicRonProvider {
    fn data_impl(
        &self,
    ) -> anyhow::Result<figment::value::Map<figment::Profile, figment::value::Dict>> {
        let version = {
            let mut max: Option<u64> = None;

            for dir_entry in fs::read_dir(&self.path)? {
                match dir_entry?.file_name().to_str() {
                    Some(filename) => match filename.strip_prefix('v') {
                        Some(version) => match version.parse::<u64>() {
                            Ok(version) => {
                                max = match max {
                                    Some(old) => {
                                        if old < version {
                                            Some(version)
                                        } else {
                                            Some(old)
                                        }
                                    }
                                    None => Some(version),
                                };
                            }
                            Err(_) => {}
                        },
                        None => {}
                    },
                    None => {}
                }
            }
            max.ok_or(anyhow!("no version found"))?
        };

        let path = self.path.join(format!("v{}", version));

        let mut f = Figment::new();

        for dir_entry in fs::read_dir(&path)? {
            let dir_entry = dir_entry?;

            let filename = dir_entry
                .file_name()
                .to_str()
                .ok_or(anyhow!("no filename"))?;

            let content = fs::read_to_string(dir_entry.path())?;

            let value: ron::Value = ron::from_str(&content)?;

            f = f.merge(figment::providers::Serialized::from(
                value,
                Profile::Default,
            ));
        }

        let data = f.data()?;

        Ok(data)
    }
}
