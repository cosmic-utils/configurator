https://github.com/facet-rs/facet/tree/main

todo:

- create schema from Shape
  - construct value from Shape.
- cosmic-panel example

// https://github.com/facet-rs/facet/blob/main/facet-format/src/serializer.rs#L2159

### issue with facet

- struct_metadata is not called with a top level tuple struct/
- Tuples and arrays are not differentiated.
- for an enum tuple variant, begin_struct and begin_seq is called