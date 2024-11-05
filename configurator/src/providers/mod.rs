use anyhow::anyhow;
use std::{fs, path::Path};

use configurator_utils::ConfigFormat;
pub use cosmic_ron::CosmicRonProvider;
use figment::{
    providers::{self, Format},
    value::Value,
    Figment, Profile, Provider,
};

mod cosmic_ron;
#[cfg(test)]
mod tests;

pub struct BoxedProvider(Box<dyn Provider>);

impl Provider for BoxedProvider {
    fn metadata(&self) -> figment::Metadata {
        self.0.metadata()
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        self.0.data()
    }

    fn profile(&self) -> Option<figment::Profile> {
        None
    }
}

#[instrument(skip_all)]
pub fn read_from_format<P: AsRef<Path>>(path: P, format: &ConfigFormat) -> BoxedProvider {
    debug!("{:?}:{}", path.as_ref(), format);

    match format {
        ConfigFormat::Json => BoxedProvider(Box::new(providers::Json::file(path))),
        ConfigFormat::CosmicRon => BoxedProvider(Box::new(
            crate::providers::CosmicRonProvider::new(path.as_ref()),
        )),
    }
}

pub fn write<P: AsRef<Path>>(path: P, format: &ConfigFormat, data: &Value) -> anyhow::Result<()> {
    // dbg!(&data);
    match format {
        ConfigFormat::Json => {
            let content = json::to_string_pretty(&data)?;
            write_and_create_parent(path, &content)?;
        }
        ConfigFormat::CosmicRon => {
            if let Some(dict) = data.as_dict() {
                for (key, value) in dict {
                    let content =
                        ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::new()).unwrap();
                    write_and_create_parent(path.as_ref().join(key), &content)?;
                }
            }
        }
    }

    Ok(())
}

fn write_and_create_parent<P: AsRef<Path>, C: AsRef<[u8]>>(
    path: P,
    contents: C,
) -> anyhow::Result<()> {
    if !path.as_ref().exists() {
        let parent = path.as_ref().parent().ok_or(anyhow!("no parent"))?;
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)?;

    Ok(())
}
