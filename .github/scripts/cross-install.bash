#!/usr/bin/env bash

# Install Cross using the pre-compiled binaries to speed up release/build time

set -e
set -u
set -o pipefail


CROSS_VERSION="v0.2.5"

cross_download_dir="$RUNNER_TEMP/cross-download"
mkdir "$cross_download_dir"
echo "$cross_download_dir" >> "$GITHUB_PATH"
cd "$cross_download_dir"
curl -LO "https://github.com/cross-rs/cross/releases/download/$CROSS_VERSION/cross-x86_64-unknown-linux-musl.tar.gz"
tar xf "cross-x86_64-unknown-linux-musl.tar.gz"
