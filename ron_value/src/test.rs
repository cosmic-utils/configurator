use std::assert_matches;

use std::fmt::Debug;

use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{F32, F64, Number, Value};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Complex {
    x: String,
    y: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum EnumComplex {
    A,
    B(i32),
    C(Complex),
    D { a: i32, b: Complex },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestEnumComplex {
    x: EnumComplex,
}

fn roundtrip<T>(data: T)
where
    T: Serialize + DeserializeOwned + Debug + PartialEq,
{
    let str_from_serde = ron::ser::to_string(&data).unwrap();

    {
        let data2: T = ron::from_str(&str_from_serde).unwrap();
        assert_eq!(data, data2);
    }

    dbg!(&str_from_serde);

    let value = crate::from_str(&str_from_serde).unwrap();

    dbg!(&value);

    let str_from_value = crate::to_string(&value).unwrap();

    dbg!(&str_from_value);

    let data2: T = ron::from_str(&str_from_value).unwrap();
    assert_eq!(data, data2);
}

#[test]
fn enum_tuple() {
    roundtrip(EnumComplex::C(Complex {
        x: "hello\n".into(),
        y: 1,
    }));
}

#[test]
fn enum_struct() {
    roundtrip(EnumComplex::D {
        a: 1,
        b: Complex {
            x: "hello".into(),
            y: 1,
        },
    });
}

#[test]
fn float_std() {
    let v = crate::from_str("1.0").unwrap();

    assert_eq!(v, Value::Number(Number::F64(F64(1.0))));
}

#[test]
fn float_frac() {
    let v = crate::from_str(".1").unwrap();

    assert_eq!(v, Value::Number(Number::F64(F64(0.1))));
}

#[test]
fn integer() {
    let v = crate::from_str("1").unwrap();

    assert_eq!(v, Value::Number(Number::U128(1)));
}

#[test]
fn integer_neg() {
    let v = crate::from_str("-1").unwrap();

    assert_eq!(v, Value::Number(Number::I128(-1)));
}


#[test]
fn list1() {
    let v = crate::from_str("[\n\"a\"]").unwrap();

    assert_eq!(v, Value::List(vec![
        Value::from("a"),
    ]));
}


#[test]
fn list() {
    let v = crate::from_str("[\n\t\"com.system76.CosmicAppList\",\n\t\"com.system76.CosmicPanelAppButton\",\n\t\"com.system76.CosmicAppletMinimize\",\n]").unwrap();

    assert_eq!(v, Value::List(vec![
        Value::from("com.system76.CosmicAppList"),
        Value::from("com.system76.CosmicPanelAppButton"),
        Value::from("com.system76.CosmicAppletMinimize"),
    ]));
}
