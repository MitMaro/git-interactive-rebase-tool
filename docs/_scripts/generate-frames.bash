#!/usr/bin/env bash

# usage generate-frames.bash file-without-extension

name="$1"
set -x
rm -rf frames/
mkdir -p frames/
ffmpeg -i "${name}.mp4" -r 20 'frames/frame-%03d.png'
