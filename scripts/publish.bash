#!/usr/bin/env bash

if [[ -z "$PUBLISH" ]]; then
	1>&2 echo "Set PUBLISH environment variable to publish"
	exit 1
fi

set -euo pipefail

if [[ ! -f "$PWD/Cargo.lock" ]]; then
	1>&2 echo "This script must be run from the project root directory"
	exit 1
fi

# order is based on dependency graph
crates=(
	"src/git"
	"src/config"
	"src/display"
	"src/todo_file"
	"src/runtime"
	"src/input"
	"src/view"
	"src/core"
	"src"
)

for crate in "${crates[@]}"; do
	(
		cd "$crate"
		1>&2 echo "Publishing $crate"
		cargo publish "$@"
		1>&2 echo "$crate published"
		1>&2 echo
		sleep 10
	) || true
done
