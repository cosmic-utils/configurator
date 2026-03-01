use std::collections::BTreeMap;

use facet::{Def, EnumType, Facet, Field, Shape, StructKind, StructType, Type, UserType};

use crate::{
    EnumVariantKind, NumberKind, RustSchema, RustSchemaId, RustSchemaKind, RustSchemaOrRef,
    RustSchemaRoot, Value,
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
            Def::Undefined => match &shape.ty {
                Type::User(UserType::Struct(st)) => {
                    self.schema_for_struct(shape, st, description, default)
                }
                Type::User(UserType::Enum(en)) => {
                    self.schema_for_enum(shape, en, description, default)
                }
                _ => {
                    todo!()
                }
            },
            Def::Map(map_def) => RustSchemaOrRef::schema(RustSchema {
                description,
                kind: RustSchemaKind::Map(self.schema_for_shape(map_def.v)),
                default,
            }),
            Def::Set(set_def) => RustSchemaOrRef::schema(RustSchema {
                description,
                kind: RustSchemaKind::Array(self.schema_for_shape(set_def.t)),
                default,
            }),
            Def::List(list_def) => RustSchemaOrRef::schema(RustSchema {
                description,
                kind: RustSchemaKind::Array(self.schema_for_shape(list_def.t)),
                default,
            }),
            Def::Array(array_def) => RustSchemaOrRef::schema(RustSchema {
                description,
                kind: RustSchemaKind::Array(self.schema_for_shape(array_def.t)),
                default,
            }),
            Def::NdArray(nd_array_def) => todo!(),
            Def::Slice(slice_def) => todo!(),
            Def::Option(option_def) => RustSchemaOrRef::schema(RustSchema {
                description,
                kind: RustSchemaKind::Option(self.schema_for_shape(option_def.t)),
                default,
            }),
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

            "()" => Some(RustSchemaKind::Unit),
            // Unknown scalar - no type constraint
            _ => None,
        }
    }

    fn schema_for_struct(
        &mut self,
        shape: &Shape,
        struct_type: &StructType,
        description: Option<String>,
        default: Option<Value>,
    ) -> RustSchemaOrRef {
        match struct_type.kind {
            StructKind::Unit => {
                self.in_progress.push(shape.type_identifier);

                let schema = RustSchema {
                    default,
                    description,
                    kind: RustSchemaKind::Struct(
                        shape.type_identifier.to_string(),
                        BTreeMap::new(),
                    ),
                };

                let ref_ = get_ref(shape);
                self.defs.insert(ref_.clone(), schema);

                RustSchemaOrRef::ref_(ref_)
            }
            StructKind::Tuple => {
                let items: Vec<RustSchemaOrRef> = struct_type
                    .fields
                    .iter()
                    .map(|f| self.schema_for_shape(f.shape.get()))
                    .collect();

                RustSchemaOrRef::schema(RustSchema {
                    default,
                    description,
                    kind: RustSchemaKind::Tuple(items),
                })
            }
            StructKind::TupleStruct => {
                self.in_progress.push(shape.type_identifier);

                let items: Vec<RustSchemaOrRef> = struct_type
                    .fields
                    .iter()
                    .map(|f| self.schema_for_shape(f.shape.get()))
                    .collect();

                let schema = RustSchema {
                    default,
                    description,
                    kind: RustSchemaKind::TupleStruct(shape.type_identifier.to_string(), items),
                };

                let ref_ = get_ref(shape);
                self.defs.insert(ref_.clone(), schema);

                RustSchemaOrRef::ref_(ref_)
            }
            StructKind::Struct => {
                self.in_progress.push(shape.type_identifier);

                let mut properties = BTreeMap::new();
                let mut required = Vec::new();

                for field in struct_type.fields {
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

                let schema = RustSchema {
                    default,
                    description,
                    kind: RustSchemaKind::Struct(shape.type_identifier.to_string(), properties),
                };

                let ref_ = get_ref(shape);
                self.defs.insert(ref_.clone(), schema);

                RustSchemaOrRef::ref_(ref_)
            }
        }
    }

    fn schema_for_enum(
        &mut self,
        shape: &Shape,
        enum_type: &EnumType,
        description: Option<String>,
        default: Option<Value>,
    ) -> RustSchemaOrRef {
        self.in_progress.push(shape.type_identifier);

        let variants = enum_type
            .variants
            .iter()
            .map(|v| {
                let variant_name = v.effective_name().to_string();

                let kind = match v.data.kind {
                    StructKind::Unit => EnumVariantKind::Unit,
                    StructKind::TupleStruct => EnumVariantKind::Tuple(
                        v.data
                            .fields
                            .iter()
                            .map(|f| self.schema_for_shape(f.shape()))
                            .collect(),
                    ),
                    StructKind::Struct => EnumVariantKind::Struct(
                        v.data
                            .fields
                            .iter()
                            .map(|f| (f.name.to_string(), self.schema_for_shape(f.shape())))
                            .collect(),
                    ),
                    StructKind::Tuple => todo!(),
                };

                (variant_name, kind)
            })
            .collect();

        let schema = RustSchema {
            default,
            description,
            kind: RustSchemaKind::Enum(shape.type_identifier.to_string(), variants),
        };

        let ref_ = get_ref(shape);
        self.defs.insert(ref_.clone(), schema);

        RustSchemaOrRef::ref_(ref_)
    }
}

fn get_ref(shape: &Shape) -> String {
    let type_name = shape.type_identifier;
    format!("#/$defs/{}", type_name)
}
