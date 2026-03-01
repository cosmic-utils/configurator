use facet::Facet;
use serde::{Deserialize, Serialize};

#[derive(Facet, Debug, Default)]
pub struct DefaultTraitStruct {
    #[facet(default)]
    x: i32,
}

#[derive(Facet, Debug)]
pub struct DefaultCustomStruct {
    #[facet(default = 3)]
    x: i32,
}

#[derive(Facet, Debug, Default)]
#[facet(default)]
#[repr(u8)]
pub enum DefaultTraitEnum {
    #[default]
    A,
    B,
}

#[test]
fn default_trait_struct() {
    dbg!(&DefaultTraitStruct::SHAPE);
}

#[test]
fn default_custom_struct() {
    dbg!(&DefaultCustomStruct::SHAPE);
}

#[test]
fn default_trait_enum() {
    dbg!(&DefaultTraitEnum::SHAPE);
}

// #[test]
// fn json() {
//     let t: DefaultTrait = facet_json::from_str("{}").unwrap();
//     let t2: DefaultTrait = json::from_str("{}").unwrap();
//     dbg!(&t, &t2);
// }

// #[derive(Facet, Debug, Default)]
// struct A {
//     x: i32,
// }

// #[test]
// fn test() {
//     let t: A = facet_json::from_str("{}").unwrap();
//     dbg!(&t);
// }
