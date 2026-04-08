#!/bin/bash -xe

# rm -rf cosmic-panel
# git clone https://github.com/wiiznokes/cosmic-panel.git --branch schema3
cd cosmic-panel
cargo run --bin gen_schema

