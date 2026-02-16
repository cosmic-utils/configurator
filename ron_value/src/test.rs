use serde::{Deserialize, Serialize};

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

impl Default for EnumComplex {
    fn default() -> Self {
        Self::C(Complex {
            x: "hello".into(),
            y: 1,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TestEnumComplex {
    x: EnumComplex,
}

#[test]
fn roundtrip() {
    let data = EnumComplex::default();

    let str = ron::to_string(&data).unwrap();

    {
        let data2: EnumComplex = ron::from_str(&str).unwrap();
        assert_eq!(data, data2);
    }

    dbg!(&str);

    let value = crate::from_str(&str).unwrap();

    dbg!(&value);

    let str2 = crate::to_string(&value).unwrap();

    dbg!(&str2);

    let data2: EnumComplex = ron::from_str(&str2).unwrap();
    assert_eq!(data, data2);
}
