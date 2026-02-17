use anyhow::anyhow;
use std::{fs, path::Path};

use configurator_utils::ConfigFormat;

use crate::generic_value::Value;

mod cosmic_ron;
// #[cfg(test)]
// mod tests;

#[instrument(skip_all)]
pub fn read_from_format(path: &Path, format: &ConfigFormat) -> Value {
    debug!("{:?}:{}", path, format);

    match format {
        ConfigFormat::Json => todo!(),
        ConfigFormat::CosmicRon => cosmic_ron::read(path),
    }
}

pub fn write(path: &Path, format: &ConfigFormat, data: &Value) -> anyhow::Result<()> {
    // dbg!(&data);
    match format {
        ConfigFormat::Json => {
            // let content = json::to_string_pretty(&data)?;
            // write_and_create_parent(path, content.as_bytes())?;
            todo!()
        }
        ConfigFormat::CosmicRon => cosmic_ron::write(path, data)?,
    }

    Ok(())
}

fn write_and_create_parent(path: &Path, contents: &[u8]) -> anyhow::Result<()> {
    if !path.exists() {
        let parent = path.parent().ok_or(anyhow!("no parent"))?;
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)?;

    Ok(())
}
