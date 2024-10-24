#![allow(clippy::type_complexity)]

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
struct Config {
    opt: Option<(Vec<String>, Vec<String>)>,
    // a: f32,
    // hash_map: HashMap<String, Complex>,
    // vec: Vec<Complex>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct Complex {
    str: String,
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
