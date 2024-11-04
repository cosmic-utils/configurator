use std::{marker::PhantomData, sync::LazyLock};

use configurator_utils::ConfigFormat;
use figment::{
    value::{Tag, Value},
    Profile, Provider,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct Config1 {
    bool: bool,
}

impl Config1 {
    fn new() -> Self {
        Self {
            bool: Default::default(),
        }
    }
}

static TESTS: LazyLock<Vec<(Value, &'static str, &'static ConfigFormat)>> = LazyLock::new(|| {
    vec![(
        Value::serialize(Config1::new()).unwrap(),
        "tests/cosmic_ron/config1",
        &ConfigFormat::CosmicRon,
    )]
});

#[test]
fn write_and_read() {
    for (initial_value, path, format) in TESTS.iter() {
        super::write(path, format, initial_value).unwrap();

        let value = super::read_from_format(path, format);

        let value = value.data().unwrap().remove(&Profile::Default).unwrap();

        let value = Value::Dict(Tag::Default, value);

        assert_eq!(initial_value, &value);
    }
}
