#!/usr/bin/env bash

set -e
set -u
set -o pipefail

cargo build --release
cargo deb

cp target/debian/git-interactive-rebase-tool_*_amd64.deb target/debian/git-interactive-rebase-tool_latest_amd64.deb
