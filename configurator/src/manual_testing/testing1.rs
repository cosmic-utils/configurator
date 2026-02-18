#![allow(clippy::type_complexity)]
#![allow(unreachable_code)]

use std::{collections::HashMap, fmt::Debug};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de};

use crate::{node::NodeContainer, test_common::EnumComplex};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Default)]
#[serde(default)]
struct Config {
    x: EnumComplex,
}

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
fn print_schema() {
    super::print_schema::<Config>(NAME);
}

#[test]
#[ignore]
fn print_node_container() {
    super::print_node_container::<Config>(NAME);
}
