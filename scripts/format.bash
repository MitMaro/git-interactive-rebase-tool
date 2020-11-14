#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update nightly
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt --all -- --check

