#!/usr/bin/env bash

# Build archive directory, copy contents and create archive

set -e
set -u
set -o pipefail

mkdir -p "$ARCHIVE"
cp "$BIN.exe" "$ARCHIVE"/
cp {CHANGELOG.md,README.md,COPYING} "$ARCHIVE"/
cp -r readme/ "$ARCHIVE"

7z a "$ARCHIVE.zip" "$ARCHIVE"
echo "ASSET=$ARCHIVE.zip" >> "$GITHUB_ENV"
