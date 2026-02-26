use std::collections::{BTreeMap, HashMap};

mod gen_schema;
mod serialize;
#[cfg(test)]
mod testing;

mod number;
mod value;

pub use number::Number;
pub use value::Value;

struct RustSchemaRoot {
    schema: RustSchemaOrRef,
    definitions: HashMap<RustSchemaId, RustSchema>,
}

struct RustSchema {
    kind: RustSchemaKind,
    default: Option<Value>,
}

type RustSchemaId = u64;

enum RustSchemaOrRef {
    Ref(RustSchemaId),
    Schema(Box<RustSchema>),
}

enum RustSchemaKind {
    Unit,
    Bool,
    Number,
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

enum EnumVariantKind {
    Unit,
    Tuple(Vec<RustSchemaOrRef>),
    Struct(BTreeMap<String, RustSchemaOrRef>),
}
