#!/usr/bin/env bash

set -euo pipefail

: '
This script will add to all crates a full list of enabled lints.
'

files=(
	'src/main.rs'
	'src/config/src/lib.rs'
	'src/core/src/lib.rs'
	'src/display/src/lib.rs'
	'src/git/src/lib.rs'
	'src/input/src/lib.rs'
	'src/runtime/src/lib.rs'
	'src/todo_file/src/lib.rs'
	'src/view/src/lib.rs'
)

# traverse parents until project root is found
project_root=
path="$PWD"
while [[ "$path" != "/" ]]; do
	if [[ -f "$path/Cargo.lock" ]]; then
		project_root="$path"
		break;
	else
		path="$(dirname "$path")"
	fi
done
if [[ -z "$project_root" ]]; then
	error "Project root could not be found"
	exit 1
fi

content="$(cat "$project_root/scripts/data/lints.rs")"
content="${content//$'\n'/\\n}"

for f in "${files[@]}"; do
	awk -i inplace '
		BEGIN       {p=1}
		/^\/\/ LINT-REPLACE-START/   {
			print;
			print "'"${content}"'";
			p=0
		}
		/^\/\/ LINT-REPLACE-END/     {p=1}
		p
	' "$project_root/$f"
done
