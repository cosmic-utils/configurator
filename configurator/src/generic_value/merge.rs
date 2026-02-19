use super::*;

impl Value {
    /// Merge two generic values together giving priority to `other` on
    /// conflicts.
    pub fn merge(&self, other: &Self) -> Self {
        match (self, other) {
            // empty is neutral
            (_, Value::Empty) => self.clone(),
            (Value::Empty, _) => other.clone(),

            // options: other has priority, but if both have Some we recursively
            // merge the inner contents.
            (Value::Option(a), Value::Option(b)) => {
                let result = match (a.as_ref(), b.as_ref()) {
                    (_, None) => a.clone(), // other == None -> keep self
                    (None, Some(bv)) => Some(Box::new((**bv).clone())),
                    (Some(av), Some(bv)) => Some(Box::new(av.merge(bv))),
                };
                Value::Option(result)
            }

            // maps use generic keys
            (Value::Map(m1), Value::Map(m2)) => {
                let mut merged = m1.clone();
                for (k, v2) in m2.0.iter() {
                    if let Some(v1) = merged.0.get(k) {
                        merged.0.insert(k.clone(), v1.merge(v2));
                    } else {
                        merged.0.insert(k.clone(), v2.clone());
                    }
                }
                Value::Map(merged)
            }

            // structs keyed by string are special case of map with an optional tag
            (Value::Struct(id1, m1), Value::Struct(id2, m2)) => {
                let mut merged = m1.clone();
                for (k, v2) in m2.0.iter() {
                    if let Some(v1) = merged.0.get(k) {
                        merged.0.insert(k.clone(), v1.merge(v2));
                    } else {
                        merged.0.insert(k.clone(), v2.clone());
                    }
                }
                // prefer other's tag if present
                let id = id2.clone().or_else(|| id1.clone());
                Value::Struct(id, merged)
            }

            // lists and tuples we merge indexwise, but elements in `other` win
            (Value::List(l1), Value::List(l2)) => {
                let len = usize::max(l1.len(), l2.len());
                let mut out = Vec::with_capacity(len);
                for i in 0..len {
                    let v = match (l1.get(i), l2.get(i)) {
                        (Some(a), Some(b)) => a.merge(b),
                        (Some(a), None) => a.clone(),
                        (None, Some(b)) => b.clone(),
                        _ => Value::Empty,
                    };
                    out.push(v);
                }
                Value::List(out)
            }

            (Value::Tuple(t1), Value::Tuple(t2)) => {
                let len = usize::max(t1.len(), t2.len());
                let mut out = Vec::with_capacity(len);
                for i in 0..len {
                    let v = match (t1.get(i), t2.get(i)) {
                        (Some(a), Some(b)) => a.merge(b),
                        (Some(a), None) => a.clone(),
                        (None, Some(b)) => b.clone(),
                        _ => Value::Empty,
                    };
                    out.push(v);
                }
                Value::Tuple(out)
            }

            (Value::NamedTuple(name1, t1), Value::NamedTuple(name2, t2)) if name1 == name2 => {
                let len = usize::max(t1.len(), t2.len());
                let mut out = Vec::with_capacity(len);
                for i in 0..len {
                    let v = match (t1.get(i), t2.get(i)) {
                        (Some(a), Some(b)) => a.merge(b),
                        (Some(a), None) => a.clone(),
                        (None, Some(b)) => b.clone(),
                        _ => Value::Empty,
                    };
                    out.push(v);
                }
                Value::NamedTuple(name1.clone(), out)
            }

            // same variant but no special merging strategy -- other wins
            (_, _) => other.clone(),
        }
    }
}

// add some basic unit tests for the new merge logic
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_scalars() {
        assert_eq!(
            Value::Bool(true).merge(&Value::Bool(false)),
            Value::Bool(false)
        );
        assert_eq!(
            Value::String("a".into()).merge(&Value::String("b".into())),
            Value::String("b".into())
        );
        assert_eq!(Value::Empty.merge(&Value::Bool(true)), Value::Bool(true));
        assert_eq!(Value::Bool(true).merge(&Value::Empty), Value::Bool(true));
    }

    #[test]
    fn merge_option() {
        let a = Value::Option(Some(Box::new(Value::Number(Number::I32(1)))));
        let b = Value::Option(Some(Box::new(Value::Number(Number::I32(2)))));
        assert_eq!(a.merge(&b), b);

        let none: Value = Value::Option(None);
        assert_eq!(a.merge(&none), a);
        assert_eq!(none.merge(&a), a);
    }

    #[test]
    fn merge_maps_and_structs() {
        let mut m1 = Map::new();
        m1.0.insert(Value::from("x"), Value::from(1));
        let mut m2 = Map::new();
        m2.0.insert(Value::from("y"), Value::from(2));
        m2.0.insert(Value::from("x"), Value::from(3));
        let merged = Value::Map(m1.clone()).merge(&Value::Map(m2.clone()));
        if let Value::Map(m) = merged {
            assert_eq!(m.0.get(&Value::from("x")), Some(&Value::from(3)));
            assert_eq!(m.0.get(&Value::from("y")), Some(&Value::from(2)));
        } else {
            panic!("expected map");
        }

        let mut s1 = Map::new();
        s1.0.insert("a".to_string(), Value::from(1));
        let mut s2 = Map::new();
        s2.0.insert("b".to_string(), Value::from(2));
        s2.0.insert("a".to_string(), Value::from(3));
        let merged = Value::Struct(None, s1).merge(&Value::Struct(None, s2));
        if let Value::Struct(_, ms) = merged {
            assert_eq!(ms.0.get("a"), Some(&Value::from(3)));
            assert_eq!(ms.0.get("b"), Some(&Value::from(2)));
        } else {
            panic!("expected struct");
        }
    }

    #[test]
    fn merge_lists() {
        let a = Value::List(vec![Value::from(1), Value::from(2)]);
        let b = Value::List(vec![Value::from(10)]);
        let merged = a.merge(&b);
        assert_eq!(merged, Value::List(vec![Value::from(10), Value::from(2)]));
    }
}
