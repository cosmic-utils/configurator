#![allow(clippy::type_complexity)]

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    opt: Option<ConfigEnum>,
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
