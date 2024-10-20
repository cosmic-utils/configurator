# WIP dconf alternative

### Note: i'm open for suggestion for the app name

Initially developed with tweaking the COSMIC(tm) desktop in mind, it could work for any client that satisfied these conditions:

- A [`Provider`](https://docs.rs/figment/latest/figment/trait.Provider.html) implementation
- A [`Serializer`](https://docs.rs/serde/latest/serde/trait.Serializer.html) implementation
- Install a JSON Schema that satisfy [the spec of this app](./SPEC.md).

Currently, only 2 crates are relevant

- configurator: the APP
- configurator_schema: the crate client can use to generate a JSON schema that respect the [spec](./SPEC.md)

zconf\* are just alternative to [cosmic-config](https://github.com/pop-os/libcosmic/tree/master/cosmic-config).

# MVP todo

- [ ] Plug the config system of COSMIC
  - [ ] Full compatibility with the config
  - [ ] Provider for ron syntax in multiple files
  - [ ] Ron serializer
  - [x] Better API to define the JSON Schema (system/home paths, ect.., define the spec)
  - [ ] Provide an option to install the schema from the app (and hopefully upsteam it when we are in a good shape)
- [ ] UI to create a value (this will be difficult, but it should be possible)
- [ ] Improve the UI a lot
- [ ] ...

## Other

- [ ] file watcher
- [ ] explore more pattern to define config in client code
- [ ] JSON Schema 1 to 1 compliance (pass the entire test suite, currently ~ 130/862 test passed)
- [ ] ...

As you see, this is quite a lot of works, so if anyone is interested, please reach out

## Credits

Created by [@wiiznokes](https://github.com/wiiznokes)
