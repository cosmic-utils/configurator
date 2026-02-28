use std::collections::{BTreeMap, HashMap};

mod gen_schema;
// mod gen_schema2;
mod serialize;
#[cfg(test)]
mod testing;

mod number;
mod value;

pub use number::Number;
pub use value::Value;

#[derive(Debug)]
pub struct RustSchemaRoot {
    pub schema: RustSchemaOrRef,
    pub definitions: BTreeMap<RustSchemaId, RustSchema>,
}

#[derive(Debug)]
pub struct RustSchema {
    pub description: Option<String>,
    pub kind: RustSchemaKind,
    pub default: Option<Value>,
}

pub type RustSchemaId = String;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum EnumVariantKind {
    Unit,
    Tuple(Vec<RustSchemaOrRef>),
    Struct(BTreeMap<String, RustSchemaOrRef>),
}

#[derive(Debug)]
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

#[cfg(test)]
mod test {
    use facet::Facet;

    use crate::gen_schema::schema_for;

    #[test]
    fn schema() {
        #[derive(Facet)]
        struct A {
            x: i32,
        }

        let schema = schema_for::<A>();

        dbg!(&schema);
    }
}
