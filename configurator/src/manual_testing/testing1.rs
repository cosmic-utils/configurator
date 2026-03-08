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

/// Doc on upper
#[derive(Clone, Debug, RustSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct Config {
    /// Doc on field x
    // x: NewStruct,
    y: Complex,
    /// Doc on field u
    u: (),
    u2: UnitS,
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
    super::from_ron::<Config>("(x:0,y:(x:\"hello\",y:10))");
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
