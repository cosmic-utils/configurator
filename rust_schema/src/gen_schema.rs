use std::collections::BTreeMap;

use facet::{Def, Facet, Field, Shape, StructKind, Type, UserType};

use crate::{
    NumberKind, RustSchema, RustSchemaId, RustSchemaKind, RustSchemaOrRef, RustSchemaRoot,
};

pub fn schema_for<T: Facet<'static>>() -> RustSchemaRoot {
    let mut ctx = SchemaContext::new();
    let schema = ctx.schema_for_shape(T::SHAPE);

    RustSchemaRoot {
        schema,
        definitions: ctx.defs,
    }
}

struct SchemaContext {
    defs: BTreeMap<RustSchemaId, RustSchema>,
    in_progress: Vec<&'static str>,
}

impl SchemaContext {
    const fn new() -> Self {
        Self {
            defs: BTreeMap::new(),
            in_progress: Vec::new(),
        }
    }

    fn schema_for_shape(&mut self, shape: &'static Shape) -> RustSchemaOrRef {
        dbg!(&shape);

        // Check for cycles - if we're already processing this type, emit a $ref
        let type_name = shape.type_identifier;
        if self.in_progress.contains(&type_name) {
            return RustSchemaOrRef::Ref(format!("#/$defs/{}", type_name));
        }

        // Build description from doc comments
        let description = if shape.doc.is_empty() {
            None
        } else {
            Some(shape.doc.join("\n").trim().to_string())
        };

        let default = None;

        // Handle the type based on its definition
        // NOTE: We check Def BEFORE shape.inner because types like Vec<T> set
        // .inner() for type parameter propagation but should still be treated
        // as List, not as transparent wrappers.
        match &shape.def {
            Def::Scalar => {
                let kind = self.schema_kind_for_scalar(shape).unwrap();
                RustSchemaOrRef::schema(RustSchema {
                    description,
                    kind,
                    default,
                })
            }
            Def::Undefined => {
                // Mark as in progress for cycle detection
                self.in_progress.push(shape.type_identifier);

                // Check if it's a struct or enum via Type
                let schema = match &shape.ty {
                    Type::User(UserType::Struct(st)) => {
                        self.schema_for_struct(shape, st.fields, st.kind, description)
                    }
                    Type::User(UserType::Enum(en)) => todo!(),
                    _ => {
                        todo!()
                    }
                };

                let ref_ = format!("#/$defs/{}", type_name);

                self.defs.insert(ref_.clone(), schema);
                RustSchemaOrRef::ref_(ref_)
            }
            Def::Map(map_def) => todo!(),
            Def::Set(set_def) => todo!(),
            Def::List(list_def) => todo!(),
            Def::Array(array_def) => todo!(),
            Def::NdArray(nd_array_def) => todo!(),
            Def::Slice(slice_def) => todo!(),
            Def::Option(option_def) => todo!(),
            Def::Result(result_def) => todo!(),
            Def::Pointer(pointer_def) => todo!(),
            Def::DynamicValue(dynamic_value_def) => todo!(),
            _ => todo!(),
        }
    }

    fn schema_kind_for_scalar(&mut self, shape: &'static Shape) -> Option<RustSchemaKind> {
        match shape.type_identifier {
            // Strings
            "String" | "str" | "&str" => Some(RustSchemaKind::String),

            // Booleans
            "bool" => Some(RustSchemaKind::Boolean),

            // Unsigned integers
            "u8" => Some(RustSchemaKind::Number(NumberKind::U8)),
            "u16" => Some(RustSchemaKind::Number(NumberKind::U16)),
            "u32" => Some(RustSchemaKind::Number(NumberKind::U32)),
            "u64" => Some(RustSchemaKind::Number(NumberKind::U64)),
            "u128" => Some(RustSchemaKind::Number(NumberKind::U128)),
            "usize" => Some(RustSchemaKind::Number(NumberKind::USize)),

            // Signed integers
            "i8" => Some(RustSchemaKind::Number(NumberKind::I8)),
            "i16" => Some(RustSchemaKind::Number(NumberKind::I16)),
            "i32" => Some(RustSchemaKind::Number(NumberKind::I32)),
            "i64" => Some(RustSchemaKind::Number(NumberKind::I64)),
            "i128" => Some(RustSchemaKind::Number(NumberKind::I128)),
            "isize" => Some(RustSchemaKind::Number(NumberKind::ISize)),

            // Floats
            "f32" => Some(RustSchemaKind::Number(NumberKind::F32)),
            "f64" => Some(RustSchemaKind::Number(NumberKind::F64)),

            // Char
            "char" => Some(RustSchemaKind::Char),

            // Unknown scalar - no type constraint
            _ => None,
        }
    }

    fn schema_for_struct(
        &mut self,
        shape: &'static Shape,
        fields: &'static [Field],
        kind: StructKind,
        description: Option<String>,
    ) -> RustSchema {
        match kind {
            StructKind::Unit => {
                // Unit struct serializes as null or empty object
                JsonSchema {
                    type_: Some(SchemaType::Null.into()),
                    description,
                    ..JsonSchema::new()
                }
            }
            StructKind::TupleStruct if fields.len() == 1 => {
                // Newtype - serialize as the inner type
                self.schema_for_shape(fields[0].shape.get())
            }
            StructKind::TupleStruct | StructKind::Tuple => {
                // Tuple struct as array - collect items for prefixItems
                let _items: Vec<JsonSchema> = fields
                    .iter()
                    .map(|f| self.schema_for_shape(f.shape.get()))
                    .collect();

                // TODO: Use prefixItems for proper tuple schema (JSON Schema 2020-12)
                JsonSchema {
                    type_: Some(SchemaType::Array.into()),
                    description,
                    ..JsonSchema::new()
                }
            }
            StructKind::Struct => {
                let mut properties = BTreeMap::new();
                let mut required = Vec::new();

                for field in fields {
                    // Skip fields marked with skip
                    if field.flags.contains(facet_core::FieldFlags::SKIP) {
                        continue;
                    }

                    let field_name = field.effective_name();
                    let field_schema = self.schema_for_shape(field.shape.get());

                    // Check if field is required (not Option and no default)
                    let is_option = matches!(field.shape.get().def, Def::Option(_));
                    let has_default = field.default.is_some();

                    if !is_option && !has_default {
                        required.push(field_name.to_string());
                    }

                    properties.insert(field_name.to_string(), field_schema);
                }

                self.in_progress.pop();

                JsonSchema {
                    type_: Some(SchemaType::Object.into()),
                    properties: Some(properties),
                    required: if required.is_empty() {
                        None
                    } else {
                        Some(required)
                    },
                    additional_properties: Some(AdditionalProperties::Bool(false)),
                    description,
                    title: Some(shape.type_identifier.to_string()),
                    ..JsonSchema::new()
                }
            }
        }
    }
}
