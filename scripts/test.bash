#!/usr/bin/env bash

set -e
set -u
set -o pipefail

cargo test -- --test-threads=1
cargo test --release -- --test-threads=1
