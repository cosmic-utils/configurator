#![allow(clippy::type_complexity)]
#![allow(unreachable_code)]

use std::{collections::HashMap, fmt::Debug};

use rust_schema2::RustSchema;
use serde::{Deserialize, Serialize, de};

use crate::{
    node::NodeContainer,
    test_common::{Complex, EnumComplex},
};

/// Doc on struct NewStruct
#[derive(Clone, Debug, RustSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct NewStruct(u32);

/// Doc on struct Complex2
#[derive(Clone, Debug, RustSchema, Serialize, Deserialize)]
#[serde(default)]
struct Complex2 {
    /// Doc on field x
    x: String,
    y: String,
}

impl Default for Complex2 {
    fn default() -> Self {
        Self {
            x: String::from("hello"),
            y: Default::default(),
        }
    }
}

/// Doc on upper
#[derive(Clone, Debug, RustSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct Config {
    /// Doc on field y
    y: Complex2,
}

#[derive(Clone, Debug, RustSchema, Serialize, Deserialize, Default)]
struct UnitS;

const NAME: &str = "testing1";

#[test]
#[ignore]
fn gen_schema() {
    super::gen_schema::<Config>(NAME);
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
fn from_ron() {
    let content = "Complex2( x: \"hello\" , y: \"\" )";

    // super::from_ron::<Complex2>(content);

    let value = ron_value::from_str(&content).unwrap();

    dbg!(&value);
}

#[test]
#[ignore]
fn print_schema() {
    super::print_schema::<Config>(NAME);
}

#[test]
#[ignore]
fn print_node_container() {
    super::print_node_container::<Config>(NAME);
}
