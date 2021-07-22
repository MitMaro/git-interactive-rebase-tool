#!/usr/bin/env bash

set -e
set -u
set -o pipefail

RUST_VERSION="nightly"

rustup update "$RUST_VERSION"
rustup component add rustfmt --toolchain "$RUST_VERSION"
cargo +"$RUST_VERSION" fmt --all -- --check

