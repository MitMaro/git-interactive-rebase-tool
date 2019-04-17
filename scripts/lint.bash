#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update stable
rustup component add clippy --toolchain stable
cargo +stable clippy --all-features -- -D warnings -A clippy::new_ret_no_self
