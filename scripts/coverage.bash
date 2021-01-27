#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update nightly
cargo +nightly install --version 0.16.0 cargo-tarpaulin
cargo +nightly tarpaulin --exclude-files=src/display/crossterm.rs --all-features --ignore-tests --line --verbose --out Html --out Lcov --output-dir coverage "$@"
