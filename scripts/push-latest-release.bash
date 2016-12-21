#!/usr/bin/env bash

set -e
set -u
set -o pipefail

REPOSITORY="MitMaro/git-interactive-rebase-tool"
TARGET_RELEASE_ID=18843342

cargo build --release --features nightly

echo "Pushing release for $OS"

if [[ "$OS" == "debian" ]]; then
    cargo deb
fi

assets="$(curl -X GET \
 -H "accept: application/vnd.github.dorian-preview+json"  \
 -H "content-type: application/json" \
 -H "authorization: token $GITHUB_ACCESS_TOKEN" \
 "https://api.github.com/repos/$REPOSITORY/releases/$TARGET_RELEASE_ID/assets" | tr -d '\n')"

assets="$(python -c "
import json
assets = json.loads('$assets')
for asset in assets:
    print asset['name'], asset['id']
")"

while read name id; do
    assetid=
    if [[ "$OS" == "debian" && "$name" == "git-interactive-rebase-tool_latest_amd64.deb" ]]; then
        assetid="$id"
    elif [[ "$OS" == "mac" && "$name" == "macos-interactive-rebase-tool" ]]; then
        assetid="$id"
    fi
    if [[ -n "$assetid" ]]; then
        curl -X DELETE \
         -H "accept: application/vnd.github.dorian-preview+json"  \
         -H "authorization: token $GITHUB_ACCESS_TOKEN" \
         "https://api.github.com/repos/$REPOSITORY/releases/assets/$assetid"
    fi
done <<< "$assets"

if [[ "$OS" == "debian" ]]; then
    curl -X POST \
     -H "accept: application/vnd.github.dorian-preview+json"  \
     -H "content-type: application/vnd.debian.binary-package" \
     -H "authorization: token $GITHUB_ACCESS_TOKEN" \
     --data-binary @"$(echo target/debian/git-interactive-rebase-tool*.deb)" \
     "https://uploads.github.com/repos/$REPOSITORY/releases/$TARGET_RELEASE_ID/assets?name=git-interactive-rebase-tool_latest_amd64.deb"
elif [[ "$OS" == "mac" ]]; then
    curl -X POST \
     -H "accept: application/vnd.github.dorian-preview+json"  \
     -H "content-type: application/x-mach-binary" \
     -H "authorization: token $GITHUB_ACCESS_TOKEN" \
     --data-binary @target/release/macos-interactive-rebase-tool \
     "https://uploads.github.com/repos/$REPOSITORY/releases/$TARGET_RELEASE_ID/assets?name=macos-interactive-rebase-tool"
fi

master_ref="$(git rev-parse origin/master)"

curl -X PATCH \
 -H "accept: application/vnd.github.dorian-preview+json"  \
 -H "content-type: application/json" \
 -H "authorization: token $GITHUB_ACCESS_TOKEN" \
 -d "{\"sha\": \"$master_ref\", \"force\": true}" \
 "https://api.github.com/repos/$REPOSITORY/git/refs/tags/latest"
