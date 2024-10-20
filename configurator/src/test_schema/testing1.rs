use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct Config {
    last_used_page: Option<String>,
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
