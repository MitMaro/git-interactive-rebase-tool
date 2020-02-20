#!/usr/bin/env bash

set -e
set -u
set -o pipefail

master_ref="$(git rev-parse "$DEFAULT_BRANCH")"
master_ref_short="$(git rev-parse --short "$DEFAULT_BRANCH")"

curl -X PATCH \
	-H "accept: application/vnd.github.dorian-preview+json"  \
	-H "content-type: application/json" \
	-H "authorization: token $GITHUB_ACCESS_TOKEN" \
	-d "{\"sha\": \"$master_ref\", \"force\": true}" \
	"https://api.github.com/repos/$REPOSITORY/git/refs/tags/latest"

curl -X PATCH \
	-H "accept: application/vnd.github.dorian-preview+json"  \
	-H "content-type: application/json" \
	-H "authorization: token $GITHUB_ACCESS_TOKEN" \
	-d "{\"name\": \"Latest Release ($master_ref_short)\"}" \
	"https://api.github.com/repos/$REPOSITORY/releases/$TARGET_RELEASE_ID"
