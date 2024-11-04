## Why `#[serde(default)]` on top of my Config struct ?

`#[serde(default)]` is mandatory because schemars will follow the serde attribute, not the `Default` impl.
Technically, we also use the `Deserialize` trait to deserialize type, so we also need `#[serde(default)]` for this, but
it could be replaced by `Figment::new().merge(providers::Serialized::from(&Config::default(), "default"));` and then extract.

## Why we can't have setters on subtypes ?

Imagine we use the struct `B` two times in struct `A`, how do we know where to put it in the file ?