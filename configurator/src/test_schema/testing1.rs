use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    hash_map: HashMap<String, I32>,
    vec: Vec<I32>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct I32(i32);

impl Default for Config {
    fn default() -> Self {
        let mut hash_map = HashMap::new();

        hash_map.insert("k".into(), I32(0));

        Self {
            hash_map,
            vec: vec![I32(0), I32(12)],
        }
    }
}

const NAME: &str = "testing1";

#[test]
pub fn gen_schema() {
    super::gen_schema::<Config>(NAME);
}

#[test]
fn print_default_figment() {
    super::print_default_figment::<Config>();
}

#[test]
fn print_json() {
    super::print_json::<Config>();
}

#[test]
fn print_ron() {
    super::print_ron::<Config>();
}

#[test]
fn print_schema() {
    super::print_schema::<Config>(NAME);
}
