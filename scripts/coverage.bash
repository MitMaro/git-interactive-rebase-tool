#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update nightly
cargo +nightly install cargo-tarpaulin
cargo +nightly tarpaulin --workspace --all-features --ignore-tests --line --verbose --out Html --out Lcov --output-dir coverage "$@"
