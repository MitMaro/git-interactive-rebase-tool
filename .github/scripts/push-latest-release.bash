#!/usr/bin/env bash

set -e
set -u
set -o pipefail

assets="$(curl -X GET \
	-H "accept: application/vnd.github.dorian-preview+json"  \
	-H "content-type: application/json" \
	-H "authorization: token $GITHUB_ACCESS_TOKEN" \
	"https://api.github.com/repos/$GITHUB_REPOSITORY/releases/$TARGET_RELEASE_ID/assets" | tr -d '\n')"

assets="$(python -c "
import json
assets = json.loads('$assets')
for asset in assets:
    print(asset['name'], asset['id'])
")"

while read name id; do
	if [[ "$name" == "$ASSET_NAME" ]]; then
		curl -X DELETE \
		 -H "accept: application/vnd.github.dorian-preview+json"  \
		 -H "authorization: token $GITHUB_ACCESS_TOKEN" \
		 "https://api.github.com/repos/$GITHUB_REPOSITORY/releases/assets/$id"
	fi
done <<< "$assets"

curl -X POST \
	-H "accept: application/vnd.github.dorian-preview+json"  \
	-H "content-type: $CONTENT_TYPE" \
	-H "authorization: token $GITHUB_ACCESS_TOKEN" \
	--data-binary @"$FILE_PATH" \
	"https://uploads.github.com/repos/$GITHUB_REPOSITORY/releases/$TARGET_RELEASE_ID/assets?name=$ASSET_NAME"
