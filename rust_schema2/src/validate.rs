use crate::*;

#[derive(Debug)]
pub enum DefaultConflictError<'a> {
    UnknownSchema(ResolveSchemaError),
    Conflict {
        id: String,
        upper_default: Option<&'a Value>,
        actual_default: Option<&'a Value>,
    },
}

impl From<ResolveSchemaError> for DefaultConflictError<'_> {
    fn from(value: ResolveSchemaError) -> Self {
        DefaultConflictError::UnknownSchema(value)
    }
}

impl std::fmt::Display for DefaultConflictError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DefaultConflictError::UnknownSchema(err) => err.fmt(f),
            DefaultConflictError::Conflict {
                id,
                upper_default,
                actual_default,
            } => {
                write!(
                    f,
                    "{}: Upper default is {:?} but his default is {:?}",
                    id, upper_default, actual_default
                )
            }
        }
    }
}

impl std::error::Error for DefaultConflictError<'_> {}

impl<'a> DefaultConflictError<'a> {
    fn conflict(
        id: impl Into<String>,
        upper: Option<&'a Value>,
        default: Option<&'a Value>,
    ) -> Self {
        Self::Conflict {
            id: id.into(),
            upper_default: upper,
            actual_default: default,
        }
    }
}

impl RustSchemaRoot {
    pub fn assert_default_no_conflict<'a>(&'a self) -> Result<(), DefaultConflictError<'a>> {
        let schema = self.resolve_schema(&self.schema)?;


        assert_default_no_conflict(self, schema, None, true)
    }
}

fn assert_default_no_conflict<'a>(
    root: &'a RustSchemaRoot,
    schema: &'a RustSchema,
    value: Option<&'a Value>,
    first_call: bool,
) -> Result<(), DefaultConflictError<'a>> {
    match &schema.kind {
        RustSchemaKind::Option(schema) => {
            let schema = root.resolve_schema(schema)?;

            if let Some(Value::Null) = value {
                assert_default_no_conflict(root, schema, None, false)?
            } else {
                assert_default_no_conflict(root, schema, value, false)?
            }
        }
        RustSchemaKind::Array(array) => {
            if let Some(template) = &array.template {
                let schema = root.resolve_schema(template)?;

                match value {
                    Some(Value::Array(values)) => {
                        for value in values {
                            assert_default_no_conflict(root, schema, Some(value), false)?
                        }
                    }
                    None => assert_default_no_conflict(root, schema, None, false)?,
                    _ => unreachable!(),
                }
            }
        }
        RustSchemaKind::Tuple(schemas) => {
            for (i, schema) in schemas.iter().enumerate() {
                let schema = root.resolve_schema(schema)?;

                match value {
                    Some(Value::Tuple(values)) => {
                        assert_default_no_conflict(root, schema, Some(&values[i]), false)?
                    }
                    None => assert_default_no_conflict(root, schema, None, false)?,
                    _ => unreachable!(),
                }
            }
        }
        RustSchemaKind::Map(schema) => {
            let schema = root.resolve_schema(schema)?;

            match value {
                Some(Value::Map(values)) => {
                    for value in values.values() {
                        assert_default_no_conflict(root, schema, Some(value), false)?
                    }
                }
                None => assert_default_no_conflict(root, schema, None, false)?,
                _ => unreachable!(),
            }
        }
        RustSchemaKind::Struct(struct_) => {
            if value != struct_.default.as_ref() {
                return Err(DefaultConflictError::conflict(
                    format!("struct {}", struct_.name),
                    value,
                    struct_.default.as_ref(),
                ));
            }

            for (field_name, field) in &struct_.fields {
                match value {
                    Some(Value::Struct(_, values)) => {
                        let value = values.get(field_name);

                        if value != field.default.as_ref() {
                            return Err(DefaultConflictError::conflict(
                                format!("field {}.{}", struct_.name, field_name),
                                value,
                                struct_.default.as_ref(),
                            ));
                        }

                        assert_default_no_conflict(root, schema, value, false)?
                    }
                    None => assert_default_no_conflict(root, schema, None, false)?,
                    _ => unreachable!(),
                }
            }
        }
        RustSchemaKind::TupleStruct(tuple_struct) => {
            if value != tuple_struct.default.as_ref() {
                return Err(DefaultConflictError::conflict(
                    format!("tuple {}", tuple_struct.name),
                    value,
                    tuple_struct.default.as_ref(),
                ));
            }

            for (i, schema) in tuple_struct.fields.iter().enumerate() {
                let schema = root.resolve_schema(schema)?;

                match value {
                    Some(Value::Tuple(values)) => {
                        assert_default_no_conflict(root, schema, Some(&values[i]), false)?
                    }
                    None => assert_default_no_conflict(root, schema, None, false)?,
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
                                    assert_default_no_conflict(
                                        root,
                                        schema,
                                        Some(&values[i]),
                                        false,
                                    )?
                                }
                                None => assert_default_no_conflict(root, schema, None, false)?,
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
                                        return Err(DefaultConflictError::conflict(
                                            format!(
                                                "field {}.{}.{}",
                                                enum_.name, variant.name, field_name,
                                            ),
                                            value,
                                            field.default.as_ref(),
                                        ));
                                    }

                                    assert_default_no_conflict(root, schema, value, false)?
                                }
                                None => assert_default_no_conflict(root, schema, None, false)?,
                                _ => {}
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
