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

#[test]
fn json() {
    let t: DefaultTraitStruct = facet_json::from_str("{}").unwrap();
    dbg!(&t);

    // let t2: DefaultTrait = json::from_str("{}").unwrap();
    // dbg!(&t, &t2);
}

// #[derive(Facet, Debug, Default)]
// struct A {
//     x: i32,
// }

// #[test]
// fn test() {
//     let t: A = facet_json::from_str("{}").unwrap();
//     dbg!(&t);
// }

#[test]
fn serde2() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Nested {
        x: i32,
        y: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(default)]
    struct A {
        x: i32,
        y: i32,
        z: Nested,
    }

    impl Default for A {
        fn default() -> Self {
            Self {
                x: 6,
                y: 3,
                z: Nested { x: 1, y: 4 },
            }
        }
    }

    let a: A = json::from_str("{\"x\":1}").unwrap();

    dbg!(&a);
}

#[test]
fn serde() {
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(default)]
    struct A {
        x: i32,
        y: i32,
    }

    impl Default for A {
        fn default() -> Self {
            Self { x: 2, y: 3 }
        }
    }

    let a: A = json::from_str("{\"x\":1}").unwrap();

    dbg!(&a);
}
