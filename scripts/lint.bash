#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update
rustup component add clippy
rustup component add rustfmt
cargo fmt --all -- --check && cargo clippy --all-features -- -D warnings -A clippy::new_ret_no_self
