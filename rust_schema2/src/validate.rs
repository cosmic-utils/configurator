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

        assert_default_no_conflict(self, schema, ValueState::NotSet)
    }
}

#[derive(Clone, Copy)]
enum ValueState<'a> {
    Some(&'a Value),
    None,
    NotSet,
}

impl<'a> From<Option<&'a Value>> for ValueState<'a> {
    fn from(value: Option<&'a Value>) -> Self {
        match value {
            Some(value) => ValueState::Some(value),
            None => ValueState::None,
        }
    }
}

impl<'a> From<ValueState<'a>> for Option<&'a Value> {
    fn from(value: ValueState<'a>) -> Self {
        match value {
            ValueState::Some(value) => Some(value),
            ValueState::None => None,
            ValueState::NotSet => panic!(),
        }
    }
}

impl<'a> From<&'a ValueState<'a>> for Option<&'a Value> {
    fn from(value: &'a ValueState<'a>) -> Self {
        match value {
            ValueState::Some(value) => Some(value),
            ValueState::None => None,
            ValueState::NotSet => panic!(),
        }
    }
}

impl<'a> ValueState<'a> {
    fn is_equal(&'a self, value: &'a Option<Value>) -> bool {
        match (self, value) {
            (ValueState::Some(v1), Some(v2)) => *v1 == v2,
            (ValueState::None, None) => true,
            (ValueState::NotSet, _) => true,
            (ValueState::Some(value), None) => false,
            (ValueState::None, Some(_)) => false,
        }
    }
}

fn assert_default_no_conflict<'a>(
    root: &'a RustSchemaRoot,
    schema: &'a RustSchema,
    value: ValueState<'a>,
) -> Result<(), DefaultConflictError<'a>> {
    match &schema.kind {
        RustSchemaKind::Option(schema) => {
            let schema = root.resolve_schema(schema)?;

            if let ValueState::Some(Value::Null) = value {
                // is it not set here ?
                assert_default_no_conflict(root, schema, ValueState::NotSet)?
            } else {
                assert_default_no_conflict(root, schema, value)?
            }
        }
        RustSchemaKind::Array(array) => {
            if let Some(template) = &array.template {
                let schema = root.resolve_schema(template)?;

                match value {
                    ValueState::Some(Value::Array(values)) => {
                        for value in values {
                            assert_default_no_conflict(root, schema, ValueState::Some(value))?
                        }
                    }
                    _ => assert_default_no_conflict(root, schema, value)?,
                }
            }
        }
        RustSchemaKind::Tuple(schemas) => {
            for (i, schema) in schemas.iter().enumerate() {
                let schema = root.resolve_schema(schema)?;

                match value {
                    ValueState::Some(Value::Tuple(values)) => {
                        assert_default_no_conflict(root, schema, ValueState::Some(&values[i]))?
                    }
                    _ => assert_default_no_conflict(root, schema, value)?,
                }
            }
        }
        RustSchemaKind::Map(schema) => {
            let schema = root.resolve_schema(schema)?;

            match value {
                ValueState::Some(Value::Map(values)) => {
                    for value in values.values() {
                        assert_default_no_conflict(root, schema, ValueState::Some(value))?
                    }
                }
                _ => assert_default_no_conflict(root, schema, value)?,
            }
        }
        RustSchemaKind::Struct(struct_) => {
            if !value.is_equal(&struct_.default) {
                return Err(DefaultConflictError::conflict(
                    format!("struct {}", struct_.name),
                    value.into(),
                    struct_.default.as_ref(),
                ));
            }

            for (field_name, field) in &struct_.fields {
                let schema = root.resolve_schema(&field.schema)?;
                let field_default_from_upper = match &struct_.default {
                    Some(struct_default) => {
                        let (_, fields) = struct_default.as_struct().unwrap();

                        fields.get(field_name)
                    }
                    None => None,
                };

                fn compare_struct_field<'a>(
                    n1: Option<&'a Value>,
                    n2: Option<&'a Value>,
                ) -> (bool, Option<&'a Value>) {
                    match (n1, n2) {
                        (None, None) => (true, None),
                        (None, Some(_)) => (true, n2),
                        (Some(_), None) => (true, n1),
                        (Some(n1), Some(n2)) => (n1 == n2, Some(n1)),
                    }
                }

                match compare_struct_field(field_default_from_upper, field.default.as_ref()) {
                    (true, value) => {
                        assert_default_no_conflict(root, schema, field_default_from_upper.into())?
                    }
                    (false, _) => {
                        return Err(DefaultConflictError::conflict(
                            format!("field {}.{}", struct_.name, field_name),
                            field_default_from_upper,
                            field.default.as_ref(),
                        ));
                    }
                }
            }
        }
        RustSchemaKind::TupleStruct(tuple_struct) => {
            if !value.is_equal(&tuple_struct.default) {
                return Err(DefaultConflictError::conflict(
                    format!("tuple {}", tuple_struct.name),
                    value.into(),
                    tuple_struct.default.as_ref(),
                ));
            }

            for (i, schema) in tuple_struct.fields.iter().enumerate() {
                let schema = root.resolve_schema(schema)?;

                match &tuple_struct.default {
                    Some(Value::Tuple(values)) => {
                        assert_default_no_conflict(root, schema, Some(&values[i]).into())?
                    }
                    None => assert_default_no_conflict(root, schema, ValueState::None)?,
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
                                ValueState::Some(Value::EnumVariantTuple(name, values))
                                    if &variant.name == name =>
                                {
                                    assert_default_no_conflict(
                                        root,
                                        schema,
                                        ValueState::Some(&values[i]),
                                    )?
                                }
                                _ => assert_default_no_conflict(root, schema, ValueState::NotSet)?,
                            }
                        }
                    }
                    EnumVariantKind::Struct(btree_map) => {
                        for (field_name, field) in btree_map {
                            let schema = root.resolve_schema(&field.schema)?;
                            match value {
                                ValueState::Some(Value::EnumVariantStruct(name, values))
                                    if &variant.name == name =>
                                {
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

                                    assert_default_no_conflict(root, schema, value.into())?
                                }
                                _ => assert_default_no_conflict(root, schema, ValueState::NotSet)?,
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
