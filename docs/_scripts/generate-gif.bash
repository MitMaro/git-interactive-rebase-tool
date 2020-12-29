#!/usr/bin/env bash

# usage generate-gif.bash <frames...> name crop
# crop value is (left,top-right,bottom)

name="$1"
crop="$2"
shift 2
set -x
convert -monitor -delay 10 -loop 0 "$@" "${name}-raw.gif"
gifsicle --crop "$crop" --colors 256 -O3  -o "${name}.gif" "${name}-raw.gif"
