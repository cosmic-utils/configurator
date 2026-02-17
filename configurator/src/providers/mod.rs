use anyhow::anyhow;
use std::{fs, path::Path};

use configurator_utils::ConfigFormat;
pub use cosmic_ron::CosmicRonProvider;

use crate::generic_value::Value;

mod cosmic_ron;
#[cfg(test)]
mod tests;

#[instrument(skip_all)]
pub fn read_from_format<P: AsRef<Path>>(path: P, format: &ConfigFormat) -> Value {
    debug!("{:?}:{}", path.as_ref(), format);

    todo!()
}

pub fn write<P: AsRef<Path>>(path: P, format: &ConfigFormat, data: &Value) -> anyhow::Result<()> {
    // dbg!(&data);
    match format {
        ConfigFormat::Json => {
            let content = json::to_string_pretty(&data)?;
            write_and_create_parent(path, &content)?;
        }
        ConfigFormat::CosmicRon => {
            todo!()
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
