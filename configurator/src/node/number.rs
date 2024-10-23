use std::fmt::Display;

use anyhow::bail;
use figment::value::Num;
use light_enum::LightEnum;

use super::NodeNumber;

#[derive(Debug, Clone, LightEnum)]
pub enum NumberValue {
    /// An 8-bit unsigned integer.
    U8(u8),
    /// A 16-bit unsigned integer.
    U16(u16),
    /// A 32-bit unsigned integer.
    U32(u32),
    /// A 64-bit unsigned integer.
    U64(u64),
    /// A 128-bit unsigned integer.
    U128(u128),
    /// An unsigned integer of platform width.
    USize(usize),
    /// An 8-bit signed integer.
    I8(i8),
    /// A 16-bit signed integer.
    I16(i16),
    /// A 32-bit signed integer.
    I32(i32),
    /// A 64-bit signed integer.
    I64(i64),
    /// A 128-bit signed integer.
    I128(i128),
    /// A signed integer of platform width.
    ISize(isize),
    /// A 32-bit wide float.
    F32(f32),
    /// A 64-bit wide float.
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

    pub fn into_num(self) -> Num {
        match self {
            NumberValue::U8(v) => Num::U8(v),
            NumberValue::U16(v) => Num::U16(v),
            NumberValue::U32(v) => Num::U32(v),
            NumberValue::U64(v) => Num::U64(v),
            NumberValue::U128(v) => Num::U128(v),
            NumberValue::USize(v) => Num::USize(v),
            NumberValue::I8(v) => Num::I8(v),
            NumberValue::I16(v) => Num::I16(v),
            NumberValue::I32(v) => Num::I32(v),
            NumberValue::I64(v) => Num::I64(v),
            NumberValue::I128(v) => Num::I128(v),
            NumberValue::ISize(v) => Num::ISize(v),
            NumberValue::F32(v) => Num::F32(v),
            NumberValue::F64(v) => Num::F64(v),
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

    pub fn try_from_figment_num(&self, value: figment::value::Num) -> anyhow::Result<NumberValue> {
        let s = match value {
            Num::U8(v) => v.to_string(),
            Num::U16(v) => v.to_string(),
            Num::U32(v) => v.to_string(),
            Num::U64(v) => v.to_string(),
            Num::U128(v) => v.to_string(),
            Num::USize(v) => v.to_string(),
            Num::I8(v) => v.to_string(),
            Num::I16(v) => v.to_string(),
            Num::I32(v) => v.to_string(),
            Num::I64(v) => v.to_string(),
            Num::I128(v) => v.to_string(),
            Num::ISize(v) => v.to_string(),
            Num::F32(v) => v.to_string(),
            Num::F64(v) => v.to_string(),
        };

        self.try_parse_from_str(&s)
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
