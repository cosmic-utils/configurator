use std::fmt::Display;

use anyhow::bail;
use light_enum::LightEnum;

use crate::generic_value::{F32, F64, Number};

use super::NodeNumber;

#[derive(Debug, Clone, LightEnum)]
pub enum NumberValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    F32(f32),
    F64(f64),
}

impl Display for NumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberValue::I128(n) => write!(f, "{}", n),
            NumberValue::F64(n) => write!(f, "{:.3}", n),
            NumberValue::U8(n) => write!(f, "{}", n),
            NumberValue::U16(n) => write!(f, "{}", n),
            NumberValue::U32(n) => write!(f, "{}", n),
            NumberValue::U64(n) => write!(f, "{}", n),
            NumberValue::U128(n) => write!(f, "{}", n),
            NumberValue::USize(n) => write!(f, "{}", n),
            NumberValue::I8(n) => write!(f, "{}", n),
            NumberValue::I16(n) => write!(f, "{}", n),
            NumberValue::I32(n) => write!(f, "{}", n),
            NumberValue::I64(n) => write!(f, "{}", n),
            NumberValue::ISize(n) => write!(f, "{}", n),
            NumberValue::F32(n) => write!(f, "{:.3}", n),
        }
    }
}

impl NumberValue {
    pub fn kind_from_str(s: &str) -> Option<NumberValueLight> {
        let e = NumberValueLight::F32;
        // note: float, and size where not tested
        let v = match s {
            "uint8" => NumberValueLight::U8,
            "uint16" => NumberValueLight::U16,
            "uint32" => NumberValueLight::U32,
            "uint64" => NumberValueLight::U64,
            "uint128" => NumberValueLight::U128,
            "usize" => NumberValueLight::USize,
            "int8" => NumberValueLight::I8,
            "int16" => NumberValueLight::I16,
            "int32" => NumberValueLight::I32,
            "int64" => NumberValueLight::I64,
            "int128" => NumberValueLight::I128,
            "isize" => NumberValueLight::ISize,
            "float" => NumberValueLight::F32,
            "float64" => NumberValueLight::F64,

            _ => return None,
        };

        Some(v)
    }

    pub fn to_number(&self) -> Number {
        match *self {
            NumberValue::U8(v) => Number::U8(v),
            NumberValue::U16(v) => Number::U16(v),
            NumberValue::U32(v) => Number::U32(v),
            NumberValue::U64(v) => Number::U64(v),
            NumberValue::U128(v) => Number::U128(v),
            NumberValue::USize(v) => Number::USize(v),
            NumberValue::I8(v) => Number::I8(v),
            NumberValue::I16(v) => Number::I16(v),
            NumberValue::I32(v) => Number::I32(v),
            NumberValue::I64(v) => Number::I64(v),
            NumberValue::I128(v) => Number::I128(v),
            NumberValue::ISize(v) => Number::ISize(v),
            NumberValue::F32(v) => Number::F32(F32(v)),
            NumberValue::F64(v) => Number::F64(F64(v)),
        }
    }

    pub fn from_number(number: &Number) -> NumberValue {
        match *number {
            Number::U8(v) => NumberValue::U8(v),
            Number::U16(v) => NumberValue::U16(v),
            Number::U32(v) => NumberValue::U32(v),
            Number::U64(v) => NumberValue::U64(v),
            Number::U128(v) => NumberValue::U128(v),
            Number::USize(v) => NumberValue::USize(v),
            Number::I8(v) => NumberValue::I8(v),
            Number::I16(v) => NumberValue::I16(v),
            Number::I32(v) => NumberValue::I32(v),
            Number::I64(v) => NumberValue::I64(v),
            Number::I128(v) => NumberValue::I128(v),
            Number::ISize(v) => NumberValue::ISize(v),
            Number::F32(v) => NumberValue::F32(v.0),
            Number::F64(v) => NumberValue::F64(v.0),
        }
    }
}

impl NodeNumber {
    pub fn new(kind: NumberValueLight) -> Self {
        Self {
            value: None,
            value_string: String::new(),
            kind,
        }
    }

    pub fn try_parse_from_str(&self, str: &str) -> anyhow::Result<NumberValue> {
        let v = match self.kind {
            NumberValueLight::U8 if let Ok(v) = str.parse::<u8>() => NumberValue::U8(v),
            NumberValueLight::U16 if let Ok(v) = str.parse::<u16>() => NumberValue::U16(v),
            NumberValueLight::U32 if let Ok(v) = str.parse::<u32>() => NumberValue::U32(v),
            NumberValueLight::U64 if let Ok(v) = str.parse::<u64>() => NumberValue::U64(v),
            NumberValueLight::U128 if let Ok(v) = str.parse::<u128>() => NumberValue::U128(v),
            NumberValueLight::USize if let Ok(v) = str.parse::<usize>() => NumberValue::USize(v),
            NumberValueLight::I8 if let Ok(v) = str.parse::<i8>() => NumberValue::I8(v),
            NumberValueLight::I16 if let Ok(v) = str.parse::<i16>() => NumberValue::I16(v),
            NumberValueLight::I32 if let Ok(v) = str.parse::<i32>() => NumberValue::I32(v),
            NumberValueLight::I64 if let Ok(v) = str.parse::<i64>() => NumberValue::I64(v),
            NumberValueLight::I128 if let Ok(v) = str.parse::<i128>() => NumberValue::I128(v),
            NumberValueLight::ISize if let Ok(v) = str.parse::<isize>() => NumberValue::ISize(v),
            NumberValueLight::F32 if let Ok(v) = str.parse::<f32>() => NumberValue::F32(v),
            NumberValueLight::F64 if let Ok(v) = str.parse::<f64>() => NumberValue::F64(v),
            _ => bail!("can't parse {} to {:?}", str, self.kind),
        };

        Ok(v)
    }
}
