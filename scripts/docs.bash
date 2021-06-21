#!/usr/bin/env bash

set -e
set -u
set -o pipefail

rustup update stable
cargo +stable doc --all-features --workspace

# try to run the nightly version, if it exists, but do not fail
(
	rustup update nightly && \
	cargo +nightly doc --all-features --workspace
) || true
