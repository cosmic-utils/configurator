rootdir := ''
prefix := '/usr'
debug := '0'


name := 'configurator'
appid := 'io.github.wiiznokes.' + name 

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-src := cargo-target-dir / if debug == '1' { 'debug' / name } else { 'release' / name }

base-dir := absolute_path(clean(rootdir / prefix))
share-dst := base-dir / 'share'

bin-dst := base-dir / 'bin' / name



run: install_schema
    cargo r --bin configurator

gen_schema:
    cargo test --package configurator config::test::gen_schema

install_schema: gen_schema
    install -Dm0644 configurator/res/{{appid}}.json ~/.local/share/configurator/{{appid}}.json


uninstall_schema:
    rm ~/.local/share/configurator/{{appid}}.json


install: install_schema
  install -Dm0755 {{bin-src}} {{bin-dst}}





# require to git clone https://github.com/json-schema-org/JSON-Schema-Test-Suite
test_suite:
    cargo test test_all_suite -- --nocapture
