---
layout: default
---

## Features

* Cross platform
* Easily `pick`, `squash`, `fixup`, `edit`, `reword`, and `drop` commits
* Reorder rebase actions
* Quickly commit changes
* Full unicode support

## Install

### Debian and derivatives

Download the `.deb` file from the [releases page][releases] and install. The executable will be installed to `/usr/bin`.

You may need to install ncurses with `apt-get install libncurses5` if it is not satisfied.

##### Configure Git

    git config --global sequence.editor interactive-rebase-tool

### MacOS and OSX

#### With Homebrew

    brew install interactive-rebase-tool

#### Without Homebrew

Download the `macos-interactive-rebase-tool` from the [releases page][releases] and copy it as `interactive-rebase-tool`
to a location on your `PATH`.

##### Configure Git

    git config --global sequence.editor interactive-rebase-tool

### FreeBSD

#### With pkg

    pkg install interactive_rebase_tool

#### From ports

    cd /usr/ports/devel/interactive_rebase_tool && make install clean

##### Configure Git

    git config --global sequence.editor interactive-rebase-tool

### Windows

*Note: Windows binaries are not fully tested. If you are having issues please report them.*

Download the tool from the [releases page][releases] and save it to a known location.

##### Configure Git

    git config --global core.editor "'C:/path/to/interactive-rebase-tool'"

[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
