#![allow(clippy::type_complexity)]
#![allow(unreachable_code)]

use std::{collections::HashMap, fmt::Debug};

use figment::value::Value;
use schemars::JsonSchema;
use serde::{de, Deserialize, Serialize};

use crate::node::NodeContainer;

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
enum ConfigEnum {
    #[default]
    A,
    B,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
struct Config {
    x: ConfigEnum,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct Complex {
    str: String,
}

const NAME: &str = "testing1";

#[test]
#[ignore]
fn gen_schema() {
    super::gen_schema::<Config>(NAME);
}

#[test]
#[ignore]
fn print_default_figment() {
    super::print_default_figment::<Config>();
}

#[test]
#[ignore]
fn print_json() {
    super::print_json::<Config>();
}

#[test]
#[ignore]
fn print_ron() {
    super::print_ron::<Config>();
}

#[test]
#[ignore]
fn print_schema() {
    super::print_schema::<Config>(NAME);
}

#[test]
#[ignore]
fn t() {
    let ron = "(x:A)";

    let c: Config = ron::from_str(ron).unwrap();
    dbg!(&c);

    let v: ValueDeserializer = ron::from_str(ron).unwrap();

    dbg!(&v);

    panic!()
}

struct ValueDeserializer {
    value: figment::value::Value,
}

impl Debug for ValueDeserializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueDeserializer")
            .field("value", &self.value)
            .finish()
    }
}

impl<'de> Deserialize<'de> for ValueDeserializer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // let node: NodeContainer = todo!();

        enum Field {
            X,
        }

        struct FieldVisitor;

        impl de::Visitor<'_> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }
        }

        struct VisitorStruct {}

        impl<'de> de::Visitor<'de> for VisitorStruct {
            type Value = ValueDeserializer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                todo!()
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                todo!()
            }
        }

        deserializer.deserialize_struct("Config", &["x"], VisitorStruct {})
    }
}
