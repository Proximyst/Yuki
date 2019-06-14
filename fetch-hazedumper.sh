#!/usr/bin/env bash

echo > src/hazedumper.rs
curl https://raw.githubusercontent.com/frk1/hazedumper/master/csgo.toml | tee build-src/hazedumper_csgo.toml
python build-src/hazedumper.py | tee src/hazedumper.rs
rustfmt src/hazedumper.rs