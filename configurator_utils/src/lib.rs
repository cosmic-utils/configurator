use std::fmt::Display;

use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    CosmicRon,
}

impl Display for ConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigFormat::Json => write!(f, "json"),
            ConfigFormat::CosmicRon => write!(f, "cosmic_ron"),
        }
    }
}

impl TryFrom<&str> for ConfigFormat {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let format = match value {
            "json" => ConfigFormat::Json,
            "cosmic_ron" => ConfigFormat::CosmicRon,
            _ => Err(anyhow!("unknown format: {}", value))?,
        };
        Ok(format)
    }
}
