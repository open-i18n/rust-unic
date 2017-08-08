#!/usr/bin/env bash

cargo run --release --package=unic-gen -- --download --generate idna ucd

echo "Generating idna tables with python"
python2 tools/gen_idna_tables.py
echo "Generating remaining ucd tables with python"
python2 tools/gen_ucd_tables.py
