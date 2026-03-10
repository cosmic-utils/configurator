use crate::*;

#[derive(Debug)]
pub enum DefaultConflictError {
    UnknownSchema(ResolveSchemaError),
    Conflict(String),
}

impl From<ResolveSchemaError> for DefaultConflictError {
    fn from(value: ResolveSchemaError) -> Self {
        DefaultConflictError::UnknownSchema(value)
    }
}

impl std::fmt::Display for DefaultConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefaultConflictError::UnknownSchema(err) => err.fmt(f),
            DefaultConflictError::Conflict(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl std::error::Error for DefaultConflictError {}

impl DefaultConflictError {
    fn conflict(error: impl Into<String>) -> Self {
        Self::Conflict(error.into())
    }
}

pub fn assert_default_no_conflict(
    root: &RustSchemaRoot,
    schema: &RustSchema,
    value: Option<&Value>,
) -> Result<(), DefaultConflictError> {
    match &schema.kind {
        RustSchemaKind::Option(schema) => {
            let schema = root.resolve_schema(schema)?;

            if let Some(Value::Null) = value {
                assert_default_no_conflict(root, schema, None)?
            } else {
                assert_default_no_conflict(root, schema, value)?
            }
        }
        RustSchemaKind::Array(array) => {
            if let Some(template) = &array.template {
                let schema = root.resolve_schema(template)?;

                match value {
                    Some(Value::Array(values)) => {
                        for value in values {
                            assert_default_no_conflict(root, schema, Some(value))?
                        }
                    }
                    None => assert_default_no_conflict(root, schema, None)?,
                    _ => unreachable!(),
                }
            }
        }
        RustSchemaKind::Tuple(schemas) => {
            for (i, schema) in schemas.iter().enumerate() {
                let schema = root.resolve_schema(schema)?;

                match value {
                    Some(Value::Tuple(values)) => {
                        assert_default_no_conflict(root, schema, Some(&values[i]))?
                    }
                    None => assert_default_no_conflict(root, schema, None)?,
                    _ => unreachable!(),
                }
            }
        }
        RustSchemaKind::Map(schema) => {
            let schema = root.resolve_schema(schema)?;

            match value {
                Some(Value::Map(values)) => {
                    for value in values.values() {
                        assert_default_no_conflict(root, schema, Some(value))?
                    }
                }
                None => assert_default_no_conflict(root, schema, None)?,
                _ => unreachable!(),
            }
        }
        RustSchemaKind::Struct(struct_) => {
            if value != struct_.default.as_ref() {
                return Err(DefaultConflictError::conflict(format!(
                    "struct {}, upper default value is {:?} but the struct default is {:?}",
                    struct_.name, value, struct_.default
                )));
            }

            for (field_name, field) in &struct_.fields {
                match value {
                    Some(Value::Struct(_, values)) => {
                        let value = values.get(field_name);

                        if value != field.default.as_ref() {
                            return Err(DefaultConflictError::conflict(format!(
                                "field {}.{}, upper default value is {:?} but the field default is {:?}",
                                struct_.name, field_name, value, struct_.default
                            )));
                        }

                        assert_default_no_conflict(root, schema, value)?
                    }
                    None => assert_default_no_conflict(root, schema, None)?,
                    _ => unreachable!(),
                }
            }
        }
        RustSchemaKind::TupleStruct(tuple_struct) => {
            if value != tuple_struct.default.as_ref() {
                return Err(DefaultConflictError::conflict(format!(
                    "tuple {}, upper default value is {:?} but the tuple default is {:?}",
                    tuple_struct.name, value, tuple_struct.default
                )));
            }

            for (i, schema) in tuple_struct.fields.iter().enumerate() {
                let schema = root.resolve_schema(schema)?;

                match value {
                    Some(Value::Tuple(values)) => {
                        assert_default_no_conflict(root, schema, Some(&values[i]))?
                    }
                    None => assert_default_no_conflict(root, schema, None)?,
                    _ => unreachable!(),
                }
            }
        }
        RustSchemaKind::Enum(enum_) => {
            for variant in &enum_.variants {
                match &variant.kind {
                    EnumVariantKind::Unit => {}
                    EnumVariantKind::Tuple(schemas) => {
                        for (i, schema) in schemas.iter().enumerate() {
                            let schema = root.resolve_schema(schema)?;

                            match value {
                                Some(Value::EnumVariantTuple(_, values)) => {
                                    assert_default_no_conflict(root, schema, Some(&values[i]))?
                                }
                                None => assert_default_no_conflict(root, schema, None)?,
                                _ => {}
                            }
                        }
                    }
                    EnumVariantKind::Struct(btree_map) => {
                        for (field_name, field) in btree_map {
                            match value {
                                Some(Value::EnumVariantStruct(_, values)) => {
                                    let value = values.get(field_name);

                                    if value != field.default.as_ref() {
                                        return Err(DefaultConflictError::conflict(format!(
                                            "field {}.{}.{}: upper default value is {:?} but the field default is {:?}",
                                            enum_.name,
                                            variant.name,
                                            field_name,
                                            value,
                                            field.default
                                        )));
                                    }

                                    assert_default_no_conflict(root, schema, value)?
                                }
                                None => assert_default_no_conflict(root, schema, None)?,
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    };

    Ok(())
}
