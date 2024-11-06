- fix slider
- push multiple on Row and Column
- add_maybe for Setting Section
- on_press_with for button
- nest #[instrument] tracing

- retrieve the actual config on the system with figment::Value
- implementer Serialize on Node
- ron::Value can't be serialized from str
  - https://github.com/ron-rs/ron/issues/189
  - https://github.com/ron-rs/ron/issues/122
- créer un nouveau type struct ValueDeserializer(NodeContainer)
et implementer Deserializer dessu.