use crate::generic_value::Value;

pub fn json_value_eq_value(json_value: &json::Value, value: &Value) -> bool {
    match (json_value, value) {
        (json::Value::Null, Value::Option(None)) => true,

        (json::Value::Bool(bool1), Value::Bool(bool2)) => bool1 == bool2,

        (json::Value::Number(num1), Value::Number(num2)) => num1.as_i64() == num2.into_f64().into(),

        (json::Value::String(str1), Value::String(str2)) => str1 == str2,

        (json::Value::Object(map1), Value::Struct(_, map2)) => {
            map1.len() == map2.len()
                && map2
                    .iter()
                    .all(|(k2, v2)| map1.get(k2).is_some_and(|v1| json_value_eq_value(v1, v2)))
        }

        // If the types do not match, return false
        _ => false,
    }
}
