use std::{
    collections::{HashMap, HashSet},
    default,
};

use facet::Facet;
use serde::{Deserialize, Serialize};

#[derive(Facet, Debug)]
pub struct SimpleStruct {
    pub u: (),
    pub opt: Option<u8>,
    pub b: bool,
    pub f: f64,
    pub i: i32,
    pub c: char,
    pub s: String,
    pub v: Vec<u8>,
    pub t: (String, String),
    pub h: HashMap<String, i32>,
    pub set: HashSet<i32>,
}

#[derive(Facet, Debug)]
pub struct UnitStruct;

#[derive(Facet, Debug)]
pub struct TupleStruct(pub String, pub i32);

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum EnumSimple {
    Unit,
    Tuple(String, i32),
    Struct { b: bool, s: String },
}

#[derive(Facet, Debug)]
pub struct Complex {
    pub s: String,
}

impl Complex {
    pub fn new(c: &str) -> Self {
        Self { s: c.into() }
    }
}

#[derive(Facet, Debug)]
pub struct TupleStruct2 {
    pub t: TupleStruct,
}

#[derive(Facet)]
pub struct NestedTuple(pub bool, pub (bool, bool));

#[derive(Facet, Debug)]
pub struct ComplexNested {
    pub c: Complex,
    pub opt_c: Option<Box<ComplexNested>>,
    pub opt_e: Option<Box<EnumNested>>,
}

impl ComplexNested {
    pub fn new(c: &str, opt_c: Option<ComplexNested>, opt_e: Option<EnumNested>) -> Self {
        Self {
            c: Complex::new(c),
            opt_c: opt_c.map(Box::new),
            opt_e: opt_e.map(Box::new),
        }
    }
}

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum EnumNested {
    Unit,
    Tuple(String, (Complex, i32)),
    Struct { c: Complex, s: String },
}

#[derive(Facet, Debug)]
pub struct StructNested {
    pub v: Vec<ComplexNested>,
    pub t: (String, ComplexNested),
    pub m: HashMap<String, ComplexNested>,
}

#[derive(Facet, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DefaultTrait {
    x: i32,
}

#[derive(Facet, Debug)]
pub struct DefaultCustom {
    #[facet(default = 3)]
    x: i32,
}

#[derive(Facet, Debug, Default)]
#[repr(u8)]
#[facet(default)]
pub enum DefaultTraitEnum {
    #[default]
    A,
    B,
}

#[test]
fn default() {
    let schema = crate::schema_for::<DefaultTraitEnum>();
    dbg!(&schema);
}

#[test]
fn json() {
    let t: DefaultTrait = facet_json::from_str("{}").unwrap();
    let t2: DefaultTrait = json::from_str("{}").unwrap();
    dbg!(&t, &t2);
}

#[derive(Facet, Debug, Default)]
struct A {
    x: i32,
}

#[test]
fn test() {
    let t: A = facet_json::from_str("{}").unwrap();
    dbg!(&t);
}
