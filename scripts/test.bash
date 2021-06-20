#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update stable
cargo test --workspace
