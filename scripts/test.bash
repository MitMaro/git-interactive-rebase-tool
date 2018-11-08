#!/usr/bin/env bash

set -e
set -u
set -o pipefail

cargo test
cargo test --release
