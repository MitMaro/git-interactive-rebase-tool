#!/usr/bin/env bash

### Instructions
# This must be run from the mitmaro/demo branch
# Create a rebase file with the contents
: "
pick f7e0f51 Add diff configuration settings
fixup a24b8e4 fixup! Add diff configuration settings
pick 30ca7cd Fix modified, should be change
pick 5f14c38 Fix typo
pick c695bff Add key bindings configuration
pick 384a40a Refactor configuration
pick ef303c7 Update variable names in git_config
pick ad7042e Add theme configuration
exec cargo build
pick 5d89b27 Add config utilities
pick 6ab2fc4 Major refactor of the configuration
"
# Window should be 11 lines tall for diff display (10 display lines + title)
# Run `xdotool getactivewindow` to fet the window for running the follow script
# Start screen recording over the window
# Run with `./demo.bash <window>`
# Wait until finish

this_window="$(xdotool getactivewindow)"
window_number="$1"
_xdotool="xdotool"

printf "Starting in 3\r";
for value in {5..1}; do
	printf "Starting in ${value}\r";
	sleep 1
done
printf "\33[2KRunning...\r"

# `xev` can be used to find key names
commands=(
	# actions
	"key;Down;0.2;3"
	"key;r;0.3"
	"key;Down;0.1"
	"key;e;0.3"
	"key;Down;0.1"
	"key;s;0.3"
	"key;Down;0.1"
	"key;f;0.3"
	"key;Down;0.1"
	"key;d;0.3"
	"sleep;0.3"
	# visual mode set action
	"key;v;0.3"
	"key;Up;0.1;4"
	"key;p;0.3"
	"key;v;0.3;2"
	"key;Down;0.2;2"
	"key;f;0.1"
	"key;j;0.2;3"
	"sleep;0.2"
	"key;k;0.2;2"
	"key;v;0.3"
	"sleep;0.3"
	# break
	"key;Down;0.1"
	"key;b;0.4;3"
	# edit
	"key;Prior;0.1;3"
	"key;Down;0.1;2"
	"sleep;0.3"
	"key;E;0.4"
	"key;BackSpace;0.1;5"
	"type;make;150"
	"sleep;0.3"
	"key;Return;0.2"
	"sleep;0.3"
	# show commit
	"key;Up;0.1"
	"key;c;0.3"
	"key;Down;0.2;4"
	"sleep;0.5"
	"key;c;0.1"
	"key;Up;0.3"
	"key;c;0"
	"key;d;0.2"
	"key;Down;0.2;14"
	"key;d;0"
	"key;c;0"
	"sleep;0.5"
	# External editor
	"key;exclam;0.3"
	"sleep;0.5"
	"key;Down;0.1"
	"key;i;0.1"
	"key;Delete;0.1;5"
	"type;drop;150"
	"key;Escape"
	"type;:wq;0.1"
	"key;Return;0.1"
)


$_xdotool windowactivate "$window_number"
sleep 1

for c in "${commands[@]}"; do
	IFS=';' read -ra p <<< "$c"
	case "${p[0]}" in
		"type")
			$_xdotool type --delay "${p[2]}" -- "${p[1]}"
			;;
		"key")
			for i in $(seq 1 "${p[3]:-1}"); do
				$_xdotool key "${p[1]}"
				sleep "${p[2]}"
			done
			;;
		"sleep")
			sleep "${p[1]}"
			;;
		*)
			echo "Invalid sequence: '${c}'"
			;;
	esac
done


xdotool windowactivate "$this_window"
printf "\33[2KDone...\n``"
