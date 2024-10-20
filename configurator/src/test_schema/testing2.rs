use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

use cosmic::iced_futures::backend::default;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// note:
// Serialize is only needed for subtype
// this is impossible to provide setters for the sub custom type
// because we don't know where the config comes from
// serde default is needed for allowing partials deserlization from file
// cosmic config probably allow need this but we should ckeck
/// Config description
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
struct Config {
    sub: SubConfig,
    choice: Choice,
    // sub_enum: EnumSubConfig,
    // float: f32,
    // active: bool,
    // opt: Option<String>,
    // vec: Vec<u32>,
    // otros: u16,
    // hella: String,
    // hash: HashMap<String, String>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(default)]
pub struct SubConfig {
    pub hella: Hella,
    pub active: bool,
    pub otros: u16,
    pub opt: Option<Option<String>>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(default)]
pub struct Hella {
    pub active: bool,
    pub otros: u16,
    pub opt: Option<Option<String>>,
    pub hella: String,
}
impl Default for Hella {
    fn default() -> Self {
        Self {
            hella: "bonjour".into(),
            active: Default::default(),
            otros: Default::default(),
            opt: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum Choice {
    #[default]
    A,
    B,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
pub enum EnumSubConfig {
    // A(A),
    B(B),
    #[default]
    C,
    D(i32),
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(default)]
pub struct A {
    s: String,
}

impl Default for A {
    fn default() -> Self {
        Self { s: "nested".into() }
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
pub struct B {}

const NAME: &str = "testing2";

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
