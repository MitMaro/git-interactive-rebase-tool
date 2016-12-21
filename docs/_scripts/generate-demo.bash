#!/usr/bin/env bash

name="$1"
set -x
rm -rf frames/
mkdir -p frames/
ffmpeg -i "${name}.mp4" -r 2 'frames/frame-%03d.png'
convert -monitor -delay 20 -loop 0 frames/*.png "${name}.gif"
gifsicle -O3 "${name}.gif" --colors 256 -o "${name}-optimized.gif"
