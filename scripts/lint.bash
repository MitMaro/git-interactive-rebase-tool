#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update stable
rustup component add clippy --toolchain stable
cargo +stable clippy --all-targets --all-features

# try to run the nightly version, if it exists, but do not fail
(
	rustup update nightly && \
	rustup component add clippy --toolchain nightly && \
	cargo +nightly clippy --all-targets --all-features
) || true
