use figment::{value::Dict, Figment, Profile, Provider};

pub fn data_default_profile_figment(figment: &Figment) -> Option<Dict> {
    // todo: support profile ?
    match figment.data() {
        Ok(mut data) => data.remove(&Profile::Default),
        Err(e) => {
            error!("can't parsing figment: {}", e);
            None
        }
    }
}

pub fn json_values_eq_figment_value<'a, I>(json_values: I, fig: &figment::value::Value) -> bool
where
    I: Iterator<Item = &'a json::Value>,
{
    for json_value in json_values {
        if match (json_value, fig) {
            (json::Value::Null, figment::value::Value::Empty(_, _)) => true,

            (json::Value::Bool(j_bool), figment::value::Value::Bool(_, f_bool)) => j_bool == f_bool,

            (json::Value::Number(j_num), figment::value::Value::Num(_, f_num)) => {
                j_num.as_i64() == f_num.to_u128_lossy().map(|e| e as i64)
            }

            (json::Value::String(j_str), figment::value::Value::String(_, f_str)) => j_str == f_str,

            // Compare objects
            (json::Value::Object(j_map), figment::value::Value::Dict(_, f_dict)) => {
                j_map.len() == f_dict.len()
                    && j_map.iter().all(|(k, j_val)| {
                        f_dict.get(k).map_or(false, |f_val| {
                            json_values_eq_figment_value(std::iter::once(j_val), f_val)
                        })
                    })
            }

            // If the types do not match, return false
            _ => false,
        } {
            return true;
        }
    }

    false
}

pub fn json_value_eq_figment_value(json_value: &json::Value, fig: &figment::value::Value) -> bool {
    match (json_value, fig) {
        (json::Value::Null, figment::value::Value::Empty(_, _)) => true,

        (json::Value::Bool(j_bool), figment::value::Value::Bool(_, f_bool)) => j_bool == f_bool,

        (json::Value::Number(j_num), figment::value::Value::Num(_, f_num)) => {
            j_num.as_i64() == f_num.to_u128_lossy().map(|e| e as i64)
        }

        (json::Value::String(j_str), figment::value::Value::String(_, f_str)) => j_str == f_str,

        // Compare objects
        (json::Value::Object(j_map), figment::value::Value::Dict(_, f_dict)) => {
            j_map.len() == f_dict.len()
                && j_map.iter().all(|(k, j_val)| {
                    f_dict
                        .get(k)
                        .map_or(false, |f_val| json_value_eq_figment_value(j_val, f_val))
                })
        }

        // If the types do not match, return false
        _ => false,
    }
}

pub fn figment_value_to_i128(value: &figment::value::Value) -> Option<i128> {
    type R = i128;
    match value.to_num()? {
        figment::value::Num::U8(x) => Some(x as R),
        figment::value::Num::U16(x) => Some(x as R),
        figment::value::Num::U32(x) => Some(x as R),
        figment::value::Num::U64(x) => Some(x as R),
        figment::value::Num::U128(x) => Some(x as R),
        figment::value::Num::USize(x) => Some(x as R),
        figment::value::Num::I8(x) => Some(x as R),
        figment::value::Num::I16(x) => Some(x as R),
        figment::value::Num::I32(x) => Some(x as R),
        figment::value::Num::I64(x) => Some(x as R),
        figment::value::Num::I128(x) => Some(x as R),
        figment::value::Num::ISize(x) => Some(x as R),
        figment::value::Num::F32(x) => Some(x as R),
        figment::value::Num::F64(x) => Some(x as R),
    }
}

pub fn figment_value_to_f64(value: &figment::value::Value) -> Option<f64> {
    type R = f64;
    match value.to_num()? {
        figment::value::Num::U8(x) => Some(x as R),
        figment::value::Num::U16(x) => Some(x as R),
        figment::value::Num::U32(x) => Some(x as R),
        figment::value::Num::U64(x) => Some(x as R),
        figment::value::Num::U128(x) => Some(x as R),
        figment::value::Num::USize(x) => Some(x as R),
        figment::value::Num::I8(x) => Some(x as R),
        figment::value::Num::I16(x) => Some(x as R),
        figment::value::Num::I32(x) => Some(x as R),
        figment::value::Num::I64(x) => Some(x as R),
        figment::value::Num::I128(x) => Some(x as R),
        figment::value::Num::ISize(x) => Some(x as R),
        figment::value::Num::F32(x) => Some(x as R),
        figment::value::Num::F64(x) => Some(x as R),
    }
}
