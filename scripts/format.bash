#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rust_version="nightly-2019-09-13"

rustup update "$rust_version"
rustup component add rustfmt --toolchain "$rust_version"
cargo +"$rust_version" fmt --all -- --check
