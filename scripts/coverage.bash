#!/usr/bin/env bash

set -e
set -u
set -o pipefail

cargo +nightly install cargo-tarpaulin
cargo +nightly tarpaulin --all-features --ignore-tests --line --verbose --out Html
