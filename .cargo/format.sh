#!/usr/bin/env bash

# Python
autopep8 --in-place tools/*.py tools/*/*.py

# Rust
cargo fmt --all
