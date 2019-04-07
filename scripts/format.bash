#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update
rustup component add rustfmt
cargo fmt --all -- --check
