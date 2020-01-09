# Installation

## Arch Linux

### Install with your AUR helper of choice

    yay -S git-interactive-rebase-tool

### Install the old fashioned way

1. Download the [package snapshot](https://aur.archlinux.org/packages/git-interactive-rebase-tool/)
1. Extract to a known location
1. Run `makepkg -si` from the extracted location

#### Troubleshooting

If you receive  the error, "no default toolchain configured", run `rustup default stable` and then retry the installation.
This generally happens when `rustup` is installed without setting a default toolchain.

### Remove

    sudo pacman -R git-interactive-rebase-tool

## Cargo Package Manager

    cargo install git-interactive-rebase-tool
    
### Remove

    cargo uninstall git-interactive-rebase-tool

## Debian and derivatives

Download the `.deb` file from the [releases page][releases] and install with:

    sudo dpkg -i /path/to/git-interactive-rebase-tool_*.deb
    
The executable will be installed to `/usr/bin`. You may need to install ncurses with `apt-get install libncursesw5` if
it is not satisfied.

### Remove

    sudo dpkg -r git-interactive-rebase-tool

## FreeBSD

### With pkg

    pkg install interactive_rebase_tool

### With ports

    cd /usr/ports/devel/interactive_rebase_tool && make install clean

### Remove

    pkg install interactive_rebase_tool

## MacOS via Homebrew

    brew install interactive-rebase-tool

### Remove

    brew rm interactive-rebase-tool

## MacOS manual install

Download the `macos-interactive-rebase-tool` from the [releases page][releases] and copy it as `interactive-rebase-tool`
to a location on your `PATH`.

### Remove

Delete the copied `interactive-rebase-tool` 

## Windows

*Note: Windows binaries are not fully tested. If you are having issues please report them.*

Download the tool from the [releases page][releases] and save it to a known location.

Or install with [Scoop][scoop]:

    scoop install git-interactive-rebase-tool

[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
[scoop]:https://scoop.sh/


