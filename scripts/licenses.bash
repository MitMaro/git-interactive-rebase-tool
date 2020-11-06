#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update nightly
cargo +nightly install cargo-about
cargo +nightly about generate "docs/licenses.hbs" > "docs/licenses.html"
