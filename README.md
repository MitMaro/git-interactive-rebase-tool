# Git Interactive Tool

Full feature terminal based sequence editor for git interactive rebase. Written in Rust using ncurses.

![Image](git-interactive-tool.gif?raw=true)

## Install

#### Debian (and similar)

Download the `.deb` file from the releases page and install. The executable will be installed to `/usr/bin`.

#### MacOS and OSX

Download the `interactive-rebase-tool` from the releases page and copy it to a location on your `PATH`.

## Configure Git

In your command line run:

    git config --global sequence.editor interactive-rebase-tool

## Usage

### Getting Help

The tool has built in help that can be accessed by hitting the `?` key.


### Key Bindings

| Key   | Description |
| ----- | ----------- |
|  Up   | Move selection up |
|  Down | Move selection Down |
|  `q`    | Abort interactive rebase |
|  `Q`    | Immediately abort interactive rebase |
|  `w`    | Write interactive rebase file |
|  `?`    | Immediately write interactive rebase file |
|  `j`    | Show help |
|  `k`    | Move selected commit up |
|  `p`    | Move selected commit down |
|  `r`    | Set selected commit to be picked |
|  `e`    | Set selected commit to be reworded |
|  `s`    | Set selected commit to be edited |
|  `f`    | Set selected commit to be squashed |
|  `x`    | Set selected commit to be fixed-up |
|  `d`    | Set selected commit to be dropped |

## License

Git Interactive Rebase Tool is released under the ISC license. See [LICENSE](LICENSE).
