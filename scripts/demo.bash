#!/usr/bin/env bash
set -x
xdotool windowactivate 6830112
sleep 0.1
xdotool type --delay 150 -- 'git rebase -i @~10'
xdotool key "Return"
sleep 1
xdotool key r
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key e
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key s
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key f
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key d
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key s
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key s
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key s
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key f
sleep 0.3
xdotool key "Down"
sleep 0.2
xdotool key r
sleep 0.3

xdotool type --delay 300 "jjjkkkk"
sleep 0.3
xdotool key "Up"
sleep 0.2
xdotool key "Up"

sleep 0.3
xdotool type --delay 300 "pjjjj"

sleep 0.1
xdotool key "Up"
sleep 0.1
xdotool key "Up"
sleep 0.1
xdotool key "Up"
sleep 0.5
xdotool key "c"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 1

xdotool key "q"
sleep 0.5
xdotool key 'question'
sleep 0.1
xdotool key "Down"
sleep 0.1
xdotool key "Down"
sleep 0.1
xdotool key "Down"
sleep 0.1
xdotool key "Down"
sleep 0.1
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 0.2
xdotool key "Down"
sleep 2
xdotool key "q"
sleep 0.5
xdotool key "w"
sleep 2
xdotool key "y"
