use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::value::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RustSchemaRoot {
    pub schema: RustSchemaOrRef,
    pub definitions: BTreeMap<RustSchemaId, RustSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RustSchema {
    pub kind: RustSchemaKind,
}

pub type RustSchemaId = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum RustSchemaOrRef {
    Ref(RustSchemaId),
    Schema(Box<RustSchema>),
}

impl RustSchemaOrRef {
    pub fn schema(schema: RustSchema) -> Self {
        Self::Schema(Box::new(schema))
    }
    pub fn ref_(ref_: RustSchemaId) -> Self {
        Self::Ref(ref_)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RustSchemaKind {
    Unit,
    Boolean,
    Number(NumberKind),
    Char,
    String,
    Option(RustSchemaOrRef),
    Array(RustSchemaOrRef),
    Tuple(Vec<RustSchemaOrRef>),
    Map(RustSchemaOrRef),

    Struct(Struct),
    TupleStruct(TupleStruct),
    Enum(Enum),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub fields: BTreeMap<String, StructField>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructField {
    pub description: Option<String>,
    pub default: Option<Value>,
    pub schema: RustSchemaOrRef,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TupleStruct {
    pub name: String,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub fields: Vec<RustSchemaOrRef>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub description: Option<String>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub description: Option<String>,
    pub kind: EnumVariantKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnumVariantKind {
    Unit,
    Tuple(Vec<RustSchemaOrRef>),
    Struct(BTreeMap<String, StructField>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NumberKind {
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    F32,
    F64,
}
