use configurator_utils::ConfigFormat;
use figment::value::Value;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Config1 {
    bool: bool,
}

impl Default for Config1 {
    fn default() -> Self {
        Self {
            bool: Default::default(),
        }
    }
}

#[test]
#[ignore = "only used for generation"]
fn gen() {
    let config = Config1::default();

    let initial_value = Value::serialize(config).unwrap();

    

    super::write(
        "src/providers/tests/cosmic_ron/config1",
        &ConfigFormat::CosmicRon,
        &initial_value,
    )
    .unwrap();
}
