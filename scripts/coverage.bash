#!/usr/bin/env bash

set -e
set -u
set -o pipefail

cargo +nightly install cargo-tarpaulin
cargo +nightly tarpaulin --exclude-files=src/display/ncurses.rs --all-features --ignore-tests --line --verbose --out Html --out Lcov --output-dir coverage
