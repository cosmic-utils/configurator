use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub last_used_page: Option<String>,
    pub cosmic_compat: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_used_page: Default::default(),
            cosmic_compat: false,
        }
    }
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use crate::app::APPID;

    use super::Config;

    #[test]
    pub fn gen_schema() {
        let path = Path::new("../configurator/res").join(format!("{}.json", APPID));

        let schema = configurator_schema::gen_schema::<Config>()
            .source_home_path(".config/configurator/configurator.json")
            .call()
            .unwrap();

        fs::write(path, &schema).unwrap();
    }
}
