#!/bin/bash -xe


git clone https://github.com/wiiznokes/cosmic-panel.git --branch schema
cd cosmic-panel
cargo test --package cosmic-panel-config --features schema gen_schema

