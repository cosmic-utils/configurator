mod figment_serde_bridge;
mod json_schema_provider;

pub use figment_serde_bridge::FigmentSerdeBridge;
pub use json_schema_provider::{json_value_to_figment_value, JsonSchemaProvider};

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use figment::{
        providers::{self, Serialized},
        Figment, Profile,
    };
    use schemars::{schema_for, JsonSchema};
    use serde::{Deserialize, Serialize};

    use crate::{FigmentSerdeBridge, JsonSchemaProvider};

    #[derive(Debug, Clone, JsonSchema, Deserialize, Serialize)]
    #[serde(default)]
    struct Config {
        field1: bool,
        sub: SubConfig,
        v: Vec<u32>,
        sub_enum: SubConfigEnum,
    }

    #[derive(Debug, Clone, JsonSchema, Deserialize, Serialize)]
    struct SubConfig {
        field1: bool,
    }

    #[derive(Debug, Clone, JsonSchema, Deserialize, Serialize)]
    enum SubConfigEnum {
        Named { hello: bool },
        None,
        NoNamed(u32),
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                field1: Default::default(),
                sub: SubConfig { field1: false },
                v: vec![1, 0],
                sub_enum: SubConfigEnum::NoNamed(90),
            }
        }
    }

    #[test]
    fn make_schema() {
        let schema = schema_for!(Config);

        let mut file = File::create("schema.json").unwrap();
        file.write_all(json::to_string_pretty(&schema).unwrap().as_bytes())
            .unwrap();
    }

    #[test]
    fn print_figment() {
        let config =
            Figment::new().merge(providers::Serialized::from(&Config::default(), "default"));

        dbg!(&config);
    }

    #[test]
    fn write_full_conf() {
        ser(&Config::default(), "default.json");
    }

    #[test]
    fn assert_schema_default_repr() {
        let schema_config = schema_for!(Config);
        let schema_figment = Figment::new().merge(JsonSchemaProvider::new(&schema_config.schema));

        let schema_serde_bridge = FigmentSerdeBridge::new(&schema_figment);

        let serde_value = json::value::to_value(Config::default()).unwrap();
        let schema_value = json::value::to_value(&schema_serde_bridge).unwrap();

        dbg!(&serde_value);
        dbg!(&schema_value);

        assert!(serde_value == schema_value);
    }

    #[test]
    fn assert_parsing() {
        let schema_config = schema_for!(Config);
        let schema_figment = Figment::new()
            .merge(JsonSchemaProvider::new(&schema_config.schema))
            .extract::<Config>()
            .unwrap();

        dbg!(&schema_figment);
    }

    #[test]
    fn merge_partial1() {
        let schema_config = schema_for!(Config);
        let default_figment = Figment::new()
            .merge(JsonSchemaProvider::new(&schema_config.schema))
            .merge(Serialized::from(partial1(), Profile::Default));

        dbg!(&default_figment);

        let serde_value = json::value::to_value(partial1()).unwrap();
        let _schema_value =
            json::value::to_value(FigmentSerdeBridge::new(&default_figment)).unwrap();

        let _e = default_figment.extract::<Config>().unwrap();

        let e = json::value::from_value::<Config>(serde_value).unwrap();

        dbg!(&e);
        // assert!(serde_value == schema_value);
    }

    fn ser<V: Serialize>(value: &V, path: &str) {
        let mut file = File::create(path).unwrap();
        file.write_all(json::to_string_pretty(value).unwrap().as_bytes())
            .unwrap();
    }

    #[test]
    fn write_partial1() {
        ser(&partial1(), "partial1.gen.json");
    }
    fn partial1() -> Config {
        Config {
            sub: SubConfig { field1: true },
            sub_enum: SubConfigEnum::Named { hello: false },
            ..Default::default()
        }
    }
}
