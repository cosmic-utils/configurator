# WIP dconf alternative

### Note: i'm open for suggestion for the app name

Initially developed with tweaking the COSMIC(tm) desktop in mind, it could work for any client that have installed [a compatible JSON Schema](./SPEC.md) and using a format compatible with this app.

Current formats supported

- Cosmic ron
- Json

Currently, only 2 crates are relevant

- configurator: the APP
- configurator_schema: the crate client can use to generate a JSON schema that respect the [spec](./SPEC.md)

zconf\* are just alternative to [cosmic-config](https://github.com/pop-os/libcosmic/tree/master/cosmic-config).

# MVP todo

- [x] Plug the config system of COSMIC
  - [x] Full compatibility with the config
  - [x] Provider for ron syntax in multiple files
  - [x] Ron serializer
  - [x] Better API to define the JSON Schema (system/home paths, ect.., define the spec)
  - [x] Provide an option to install the schema from the app (and hopefully upsteam it when we are in a good shape)
- [x] UI to create a value (this will be difficult, but it should be possible)
- [x] Improve the UI a lot
- [ ] polish the code (test more complex structure)
- [ ] release as a flatpak app

## Other

- [ ] file watcher
- [ ] explore more pattern to define config in client code
- [ ] JSON Schema 1 to 1 compliance (pass the entire test suite, currently ~ 335/862 test passed)
- [ ] ...

As you see, this is quite a lot of works, so if anyone is interested, please reach out

![](./configurator/res/screenshots/cosmic-panel-compat.png)
_All the cosmic panel config_ modifiable in the app.

## Credits

Created by [@wiiznokes](https://github.com/wiiznokes)
