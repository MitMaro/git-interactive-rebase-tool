# Installation

## Table of Contents

* [Arch Linux](./install.md#arch-linux)
* [Cargo Package Manager](./install.md#cargo-package-manager)
* [Debian and derivatives](./install.md#debian-and-derivatives)
* [FreeBSD](./install.md#freebsd)
* [macOS](./install.md#macos)
* [Windows](./install.md#windows)

## Arch Linux

### With your [AUR](https://aur.archlinux.org/) helper of choice

    yay -S git-interactive-rebase-tool

### Manual Install

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

The executable will be installed to `/usr/bin`.

### Remove

    sudo dpkg -r git-interactive-rebase-tool

## FreeBSD

### With [Ports](https://www.freebsd.org/ports/)

#### Using pkg

    pkg install interactive_rebase_tool

#### Manual

    cd /usr/ports/devel/interactive_rebase_tool && make install clean

#### Remove

    pkg remove interactive_rebase_tool

## macOS

### With [Homebrew](https://brew.sh/)

    brew install interactive-rebase-tool

#### Remove

    brew rm interactive-rebase-tool

### Manual install

Download the `macos-interactive-rebase-tool` from the [releases page][releases] and copy it as `interactive-rebase-tool`
to a location on your `PATH`.

#### Remove

Delete the copied `interactive-rebase-tool`.

### Notes

On macOS, Terminal.app does not support highlighting the selected line(s). If you want this feature you will need to use
a terminal emulator like [iTerm2](https://iterm2.com/index.html).

## Windows

### With [Chocolatey](https://chocolatey.org/)

    choco install git-interactive-rebase-tool

#### Remove

    choco uninstall git-interactive-rebase-tool

### With [Scoop](https://scoop.sh/)

    scoop install git-interactive-rebase-tool

#### Remove

    scoop uninstall git-interactive-rebase-tool

### Manual Install

*Note: Windows binaries are not fully tested. If you are having issues please report them.*

Download the tool from the [releases page][releases] and save it to a known location.

#### Remove

Delete the saved executable.

[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
