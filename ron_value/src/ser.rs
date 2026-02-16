use crate::{Number, Value};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum SerializeError {
    UnsupportedVariant,
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializeError::UnsupportedVariant => {
                write!(f, "unsupported variant for serialization")
            }
        }
    }
}

impl std::error::Error for SerializeError {}

pub fn to_string(value: &Value) -> Result<String, SerializeError> {
    match value {
        Value::Unit => Ok("()".to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Char(c) => Ok(format!("'{}'", escape_char(*c))),
        Value::Number(n) => Ok(number_to_string(n)),
        Value::String(s) => Ok(format!("\"{}\"", escape_string(s))),
        Value::Bytes(b) => Ok(format!(
            "b\"{}\"",
            escape_string(&String::from_utf8_lossy(b))
        )),
        Value::Option(opt) => match opt {
            None => Ok("None".to_string()),
            Some(v) => Ok(format!("Some({})", to_string(v)?)),
        },
        Value::List(vec) => {
            let elems: Result<Vec<_>, _> = vec.iter().map(|v| to_string(v)).collect();
            Ok(format!("[{}]", elems?.join(", ")))
        }
        Value::Map(map) => {
            let elems: Result<Vec<_>, _> = map
                .iter()
                .map(|(k, v)| Ok(format!("{}: {}", to_string(k)?, to_string(v)?)))
                .collect();
            Ok(format!("{{{}}}", elems?.join(", ")))
        }
        Value::Tuple(vec) => {
            let elems: Result<Vec<_>, _> = vec.iter().map(|v| to_string(v)).collect();
            Ok(format!("({})", elems?.join(", ")))
        }
        Value::UnitStructOrEnum(name) => Ok(name.to_string()),
        Value::UnitEnum(name) => Ok(name.to_string()),
        Value::UnitStruct(name) => Ok(format!("{}()", name)),
        Value::StructOrEnum(opt_name, map) => {
            let elems: Result<Vec<_>, _> = map
                .iter()
                .map(|(k, v)| Ok(format!("{}: {}", k, to_string(v)?)))
                .collect();
            if let Some(name) = opt_name {
                Ok(format!("{} {{ {} }}", name, elems?.join(", ")))
            } else {
                Ok(format!("{{ {} }}", elems?.join(", ")))
            }
        }
        Value::Struct(opt_name, map) => {
            let elems: Result<Vec<_>, _> = map
                .iter()
                .map(|(k, v)| Ok(format!("{}: {}", k, to_string(v)?)))
                .collect();
            if let Some(name) = opt_name {
                Ok(format!("{} {{ {} }}", name, elems?.join(", ")))
            } else {
                Ok(format!("{{ {} }}", elems?.join(", ")))
            }
        }
        Value::Enum(name, map) => {
            let elems: Result<Vec<_>, _> = map
                .iter()
                .map(|(k, v)| Ok(format!("{}: {}", k, to_string(v)?)))
                .collect();
            Ok(format!("{} {{ {} }}", name, elems?.join(", ")))
        }
        Value::EnumTuple(name, vec) => {
            let elems: Result<Vec<_>, _> = vec
                .iter()
                .map(|v| match v {
                    Value::Struct(None, map) => {
                        let inner: Result<Vec<_>, _> = map
                            .iter()
                            .map(|(k, v)| Ok(format!("{}: {}", k, to_string(v)?)))
                            .collect();
                        Ok(format!("({})", inner?.join(",")))
                    }
                    Value::Tuple(inner_vec) if inner_vec.len() == 1 => {
                        if let Value::Struct(None, map) = &inner_vec[0] {
                            let inner: Result<Vec<_>, _> = map
                                .iter()
                                .map(|(k, v)| Ok(format!("{}: {}", k, to_string(v)?)))
                                .collect();
                            Ok(format!("({})", inner?.join(",")))
                        } else {
                            Ok(to_string(v)?)
                        }
                    }
                    other => Ok(to_string(other)?),
                })
                .collect();

            Ok(format!("{}({})", name, elems?.join(", ")))
        }
    }
}

fn escape_string(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}

fn escape_char(c: char) -> String {
    match c {
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        other => other.to_string(),
    }
}

fn number_to_string(n: &Number) -> String {
    match n {
        Number::I8(v) => v.to_string(),
        Number::I16(v) => v.to_string(),
        Number::I32(v) => v.to_string(),
        Number::I64(v) => v.to_string(),
        Number::I128(v) => v.to_string(),
        Number::U8(v) => v.to_string(),
        Number::U16(v) => v.to_string(),
        Number::U32(v) => v.to_string(),
        Number::U64(v) => v.to_string(),
        Number::U128(v) => v.to_string(),
        Number::F32(v) => format!("{}", v.get()),
        Number::F64(v) => format!("{}", v.get()),
        #[cfg(not(doc))]
        Number::__NonExhaustive(_) => "0".to_string(),
    }
}
