#!/usr/bin/env bash

# Build archive directory, copy contents and create archive

set -e
set -u
set -o pipefail

mkdir -p "$ARCHIVE"
cp "$BIN" "$ARCHIVE"/
cp {CHANGELOG.md,README.md,COPYING,src/interactive-rebase-tool.1} "$ARCHIVE"/
cp -r readme/ "$ARCHIVE"

tar czf "$ARCHIVE.tar.gz" "$ARCHIVE"
echo "ASSET=$ARCHIVE.tar.gz" >> "$GITHUB_ENV"
