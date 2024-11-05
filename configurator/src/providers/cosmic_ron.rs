use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};
use figment::{value::Dict, Figment, Metadata, Profile, Provider};
use ron::Map;
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
        // dbg!(&map);

        self.data_impl().map_err(figment::Error::custom)
    }
}

impl CosmicRonProvider {
    fn data_impl(
        &self,
    ) -> anyhow::Result<figment::value::Map<figment::Profile, figment::value::Dict>> {
        // let version = {
        //     let mut max: Option<u64> = None;

        //     for dir_entry in fs::read_dir(&self.path)? {
        //         if let Some(filename) = dir_entry?.file_name().to_str() {
        //             if let Some(version) = filename.strip_prefix('v') {
        //                 if let Ok(version) = version.parse::<u64>() {
        //                     max = match max {
        //                         Some(old) => {
        //                             if old < version {
        //                                 Some(version)
        //                             } else {
        //                                 Some(old)
        //                             }
        //                         }
        //                         None => Some(version),
        //                     };
        //                 }
        //             }
        //         }
        //     }
        //     max.ok_or(anyhow!("no version found"))?
        // };

        // let path = self.path.join(format!("v{}", version));

        let mut ron_map = ron::Map::new();

        for dir_entry in fs::read_dir(&self.path)? {
            let dir_entry = dir_entry?;

            let filename = dir_entry.file_name();

            let filename = filename.to_str().ok_or(anyhow!("no filename"))?;

            let content = fs::read_to_string(dir_entry.path())?;

            debug!("{}", content);

            let value: ron::Value = ron::from_str(&content)?;

            debug!("{:?}", value);

            ron_map.insert(ron::Value::String(filename.to_string()), value);
        }

        debug!("{:?}", ron_map);

        let data = Figment::new()
            .merge(figment::providers::Serialized::from(
                ron_map,
                Profile::Default,
            ))
            .data()?;

        Ok(data)
    }
}
