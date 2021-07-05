#!/usr/bin/env bash

if [[ -z "$PUBLISH" ]]; then
	1>&2 echo "Set PUBLISH environment variable to publish"
	exit 1;
fi

set -euo pipefail

crates=(
	'src/'
	"src/config"
	"src/core"
	"src/display"
	"src/input"
	"src/todo_file"
	"src/view"
)

for crate in "${crates[@]}"; do
	(
		cd "$crate"
		1>&2 echo "Publishing $crate"
		cargo publish "$@"
		1>&2 echo "$crate published"
		1>&2 echo
	)
done
