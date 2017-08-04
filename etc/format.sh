#!/usr/bin/env bash
set -ev

# Rust
cargo fmt --all

# Python
# this goes second, since not everyone has autopep8 installed
autopep8 --in-place tools/*.py tools/*/*.py
