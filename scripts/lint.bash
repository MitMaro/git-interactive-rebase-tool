#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update
rustup component add clippy
cargo clippy --all-features -- -D warnings -A clippy::new_ret_no_self
