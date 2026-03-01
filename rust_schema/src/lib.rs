use std::collections::{BTreeMap, HashMap};

mod gen_schema;
mod serialize;

#[cfg(test)]
mod test_common;

mod number;
mod value;

use facet::Facet;
pub use number::Number;
pub use value::Value;

pub use gen_schema::{schema_for, to_schema};

#[derive(Facet, Debug)]
pub struct RustSchemaRoot {
    pub schema: RustSchemaOrRef,
    pub definitions: BTreeMap<RustSchemaId, RustSchema>,
}

#[derive(Facet, Debug)]
pub struct RustSchema {
    pub description: Option<String>,
    pub kind: RustSchemaKind,
    pub default: Option<Value>,
}

pub type RustSchemaId = String;

#[derive(Facet, Debug)]
#[repr(u8)]
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

#[derive(Facet, Debug)]
#[repr(u8)]
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
    Struct(String, BTreeMap<String, RustSchemaOrRef>),
    TupleStruct(String, Vec<RustSchemaOrRef>),
    Enum(String, Vec<(String, EnumVariantKind)>),
}

#[derive(Facet, Debug)]
#[repr(u8)]
pub enum EnumVariantKind {
    Unit,
    Tuple(Vec<RustSchemaOrRef>),
    Struct(BTreeMap<String, RustSchemaOrRef>),
}

#[derive(Facet, Debug)]
#[repr(u8)]
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
