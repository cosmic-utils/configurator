rootdir := ''
prefix := x"~/.local"
debug := '0'


name := 'configurator'
appid := 'io.github.cosmic_utils.' + name 

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-src := cargo-target-dir / if debug == '1' { 'debug' / name } else { 'release' / name }

base-dir := absolute_path(clean(rootdir / prefix))
share-dst := base-dir / 'share'

bin-dst := base-dir / 'bin' / name
desktop-dst := share-dst / 'applications' / appid + '.desktop'
icon-dst := share-dst / 'icons/hicolor/scalable/apps' / appid + '.svg'
env-dst := rootdir / 'etc/profile.d' / name + '.sh'
schema-dst := share-dst / 'configurator' / appid + '.json'

default: build-release

run:
    cargo r --bin configurator

build-debug *args:
  cargo build {{args}}

build-release *args:
  cargo build --release {{args}}


install: 
  install -Dm0755 {{bin-src}} {{bin-dst}}
  install -Dm0644 res/desktop_entry.desktop {{desktop-dst}}
  install -Dm0644 res/app_icon.svg {{icon-dst}}
  install -Dm0644 res/config_schema.json {{schema-dst}}


# call before pull request
pull: fmt prettier fix test

gen_schema:
    cargo test --package configurator config::test::gen_schema -- --ignored

uninstall:
  rm {{bin-dst}}
  rm {{desktop-dst}}
  rm {{icon-dst}}
  rm {{schema-dst}}


# require to git clone https://github.com/json-schema-org/JSON-Schema-Test-Suite
test_suite:
    cargo test test_all_suite -- --nocapture --ignored


###################  Test

test:
	cargo test --workspace --all-features

###################  Format

fix:
	cargo clippy --workspace --all-features --fix --allow-dirty --allow-staged

fmt:
	cargo fmt --all

prettier:
	# install on Debian: sudo snap install node --classic
	# npx is the command to run npm package, node is the runtime
	npx prettier -w .

# todo: add to CI when ubuntu-image get appstream version 1.0
metainfo-check:
	appstreamcli validate --pedantic --explain --strict res/metainfo.xml


# flatpak

setupf:
  rm -rf flatpak-builder-tools
  git clone https://github.com/flatpak/flatpak-builder-tools
  pip install aiohttp toml


sources_gen:
  python3 flatpak-builder-tools/cargo/flatpak-cargo-generator.py ./Cargo.lock -o cargo-sources.json

uninstallf:
  flatpak uninstall io.github.cosmic_utils.configurator -y || true

# deps: flatpak-builder git-lfs
build_and_install: uninstallf
  flatpak-builder \
    --force-clean \
    --verbose \
    --ccache \
    --user --install \
    --install-deps-from=flathub \
    --repo=repo \
    flatpak-out \
    io.github.cosmic_utils.configurator.json

runf:
  flatpak run io.github.cosmic_utils.configurator
