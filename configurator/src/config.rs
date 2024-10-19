use std::{
    collections::HashMap,
    fs::{self, File},
    path::Path,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::app::APPID;

pub const SCHEMAS_PATH: &str = "configurator/schemas";
pub const CONFIG_PATH: &str = "configurator/config";

// note:
// Serialize is only needed for subtype
// this is impossible to provide setters for the sub custom type
// because we don't know where the config comes from
// serde default is needed for allowing partials deserlization from file
// cosmic config probably allow need this but we should ckeck
/// Config description
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub float: f32,
    /// Activate something
    pub active: bool,
    pub sub: SubConfig,
    pub opt: Option<String>,
    // pub vec: Vec<u32>,
    pub otros: u16,
    // pub hella: String,
    pub choice: Choice,
    pub sub_enum: EnumSubConfig,
    // pub hash: HashMap<String, String>,
}
impl Default for Config {
    fn default() -> Self {
        // let mut hash = HashMap::new();

        // hash.insert("hello".into(), "mais non".into());

        Self {
            active: Default::default(),
            sub: SubConfig {
                hella: Hella {
                    hella: "bonjour".into(),
                },
            },
            sub_enum: Default::default(),
            choice: Choice::A,
            otros: 0,
            // hella: "hello".into(),
            opt: None,
            float: 13.2,
            // vec: vec![1],
            // hash,
        }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(default)]
pub struct SubConfig {
    // pub active: bool,
    // pub otros: u16,
    // pub opt: Option<Option<String>>,
    // pub choice: Choice,
    pub hella: Hella,
}

impl Default for SubConfig {
    fn default() -> Self {
        Self {
            hella: Hella {
                hella: "bonsoir".into(),
            },
        }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(default)]
pub struct Hella {
    // pub active: bool,
    // pub otros: u16,
    // pub opt: Option<Option<String>>,
    // pub choice: Choice,
    pub hella: String,
}
impl Default for Hella {
    fn default() -> Self {
        Self {
            hella: "niquer sa mere".into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum Choice {
    #[default]
    A,
    B,
}

#[test]
pub fn gen_schema() {
    let path = Path::new("../configurator/res").join(format!("{}.json", APPID));

    let schema = configurator_schema::gen_schema::<Config>()
        .source_home_paths(&[".config/configurator/configurator.json"])
        .call()
        .unwrap();

    fs::write(path, &schema).unwrap();
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
pub enum EnumSubConfig {
    A(A),
    B(B),
    #[default]
    C,
    D(i32),
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct A {
    s: String,
}

impl Default for A {
    fn default() -> Self {
        Self { s: "nested".into() }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
pub struct B {}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use figment::{providers, Figment, Profile};
    use schemars::schema_for;

    use crate::config::{Hella, SubConfig};

    use super::Config;

    #[test]
    fn print_default_figment() {
        let figment = Figment::new().merge(providers::Serialized::from(
            &Config::default(),
            Profile::Default,
        ));

        dbg!(&figment);
    }

    #[test]
    fn print_json() {
        let e = json::to_string_pretty(&Config::default()).unwrap();

        print!("{}", e);
    }

    #[test]
    fn print_ron() {
        let mut hash = HashMap::new();

        hash.insert(
            SubConfig {
                hella: Hella {
                    hella: "bonjour".into(),
                },
            },
            "hello".to_string(),
        );

        let e = ron::to_string(&hash).unwrap();

        print!("{}", e);
    }

    #[test]
    fn gen_schema() {
        super::gen_schema();
    }

    #[test]
    fn print_schema() {
        let schema = schema_for!(Config);

        dbg!(&schema);
    }
}
