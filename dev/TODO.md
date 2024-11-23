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

ce qui ne marche pas

- les enums ne sont pas bien serializées
- figment::Value ne se serialize pas correctement en ron. Notament les key des Dict qui ne sont mis entre guillement, meme pour les noms des fields.

comment tester ?

1. implementer CosmicConfigEntry sur les struct de Test.
2. Les écrires dans un repertoire
3. assert que le contenu est égual a la generation du provider.
