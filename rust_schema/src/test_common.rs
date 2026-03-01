use std::collections::{HashMap, HashSet};

use facet::Facet;

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
