use std::{fs::File, io::Write};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod expe_serde1;
mod expe_serde2;
mod expe_serde3;

#[derive(Debug, Clone, JsonSchema, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub private_mode: bool,
    pub maximum_entries_lifetime: Option<u64>,
    pub maximum_entries_number: Option<u32>,
    pub horizontal: bool,
    pub unique_session: bool,
    pub sub2: (bool, bool),
    pub sub: (bool, SubConfig),
    #[serde(default = "t")]
    pub my_enum: MyEnum,
}

fn t() -> MyEnum {
    MyEnum::Var1
}

#[derive(Debug, Clone, JsonSchema, Deserialize, Serialize)]
pub struct SubConfig {
    pub private_mode: bool,
    pub maximum_entries_number: Option<u32>,
    pub horizontal: bool,
    pub unique_session: bool,
}

#[derive(Debug, Clone, JsonSchema, Deserialize, Serialize)]
pub enum MyEnum {
    Var1,
    Var2(i32),
    Var3 { sub: SubConfig },
}

impl Default for Config {
    fn default() -> Self {
        Self {
            private_mode: false,
            maximum_entries_lifetime: Some(30), // 30 days,
            maximum_entries_number: Some(500),
            horizontal: false,
            unique_session: false,
            sub2: (false, true),
            sub: (
                false,
                SubConfig {
                    private_mode: false,
                    maximum_entries_number: Some(500),
                    horizontal: false,
                    unique_session: false,
                },
            ),
            my_enum: MyEnum::Var3 {
                sub: SubConfig {
                    private_mode: false,
                    maximum_entries_number: Some(500),
                    horizontal: false,
                    unique_session: true,
                },
            },
        }
    }
}

fn ser<V: Serialize>(value: &V, path: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(serde_json::to_string_pretty(value).unwrap().as_bytes())
        .unwrap();
}

fn main() {}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config2 {
    pub choice: Choice,
}

impl Default for Config2 {
    fn default() -> Self {
        Self {
            choice: Choice::A(A { a: false }),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]

pub enum Choice {
    A(A),
    B(B),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct A {
    pub a: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct B {
    pub b: bool,
}

#[cfg(test)]
mod test {
    use std::{io::Read, path::PathBuf};

    use figment::{
        providers::{self, Format, Json},
        Figment, Provider,
    };
    use schemars::schema_for;

    use super::*;

    #[test]
    fn merge() {
        let default = Config2::default();

        ser(&default, "test_2.json");
        let default2 = Config2 {
            choice: Choice::B(B { b: true }),
        };

        let config = Figment::new()
            // .merge(Json::file_exact(&PathBuf::from("merge1.json")))
            .merge(providers::Serialized::from(default, "default"))
            .merge(providers::Serialized::from(default2, "default"));

        dbg!(&config);
    }

    #[test]
    fn maine() {
        let schema = schema_for!(Config);

        let mut file = File::create("schema.json").unwrap();
        file.write_all(serde_json::to_string_pretty(&schema).unwrap().as_bytes())
            .unwrap();

        ser(&Config::default(), "config.json");

        let mut file = File::open("config2.json").unwrap();

        let mut content = String::new();

        file.read_to_string(&mut content).unwrap();

        let s = serde_json::from_str::<Config>(&content).unwrap();

        dbg!(&s);
    }

    #[test]
    fn figment_test() {
        let default = Config::default();

        let config = Figment::new()
            // .merge(Json::file_exact(&PathBuf::from("merge1.json")))
            .merge(Json::file_exact(PathBuf::from("merge2.json")))
            .merge(providers::Serialized::from(default, "default"));

        dbg!(&config);
    }

    impl Provider for Config {
        fn metadata(&self) -> figment::Metadata {
            figment::Metadata::named("name")
        }

        fn data(
            &self,
        ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error>
        {
            let map = figment::value::Map::new();

            Ok(map)
        }
    }
}
