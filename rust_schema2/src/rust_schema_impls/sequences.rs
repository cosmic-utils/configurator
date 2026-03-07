use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};

use crate::{NumberKind, RustSchema, RustSchemaKind, RustSchemaTrait, SchemaGenerator};

macro_rules! seq_impl {
    ($type:ty) => {
        impl<T: RustSchemaTrait> RustSchemaTrait for $type {
            fn schema(generator: &mut SchemaGenerator) -> RustSchema {
                RustSchema {
                    kind: RustSchemaKind::Array(generator.schema_for::<T>()),
                }
            }
        }
    };
}

macro_rules! set_impl {
    ($type:ty) => {
        impl<T: RustSchemaTrait> RustSchemaTrait for $type {
            fn schema(generator: &mut SchemaGenerator) -> RustSchema {
                RustSchema {
                    kind: RustSchemaKind::Array(generator.schema_for::<T>()),
                }
            }
        }
    };
}

seq_impl!(BinaryHeap<T>);
seq_impl!(LinkedList<T>);
seq_impl!([T]);
seq_impl!(Vec<T>);
seq_impl!(VecDeque<T>);

set_impl!(BTreeSet<T>);
set_impl!(HashSet<T>);
