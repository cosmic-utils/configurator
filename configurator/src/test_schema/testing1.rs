use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
struct Config {
    opt: Option<(Vec<String>, (Vec<String>, String))>,
    a: f32,
    // hash_map: HashMap<String, Complex>,
    // vec: Vec<Complex>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct Complex {
    str: String,
}

impl Default for Config {
    fn default() -> Self {
        // let mut hash_map = HashMap::new();
        //
        // hash_map.insert("k".into(), Complex { str: "0".into() });

        Self {
            // hash_map,
            // vec: vec![Complex { str: "0".into() }, Complex { str: "12".into() }],
            opt: None,
            a: 0.,
        }
    }
}

const NAME: &str = "testing1";

#[test]
fn gen_schema() {
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
