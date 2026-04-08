mod schema;

use std::{borrow::Cow, collections::BTreeMap};

pub use rust_schema2_derive::*;
pub use schema::*;
pub use value::*;

pub use generate::{RustSchemaTrait, SchemaGenerator, schema_for};

mod generate;
mod rust_schema_impls;
mod validate;
mod value;
