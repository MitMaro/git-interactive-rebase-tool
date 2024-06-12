# Installation

## Table of Contents

* [Official Installation Methods](#official-installation-methods)
    * [Debian and derivatives](#debian-and-derivatives)
    * [Red Hat Linux, Fedora and derivatives](#red-hat-linux-fedora-and-derivatives)
    * [macOS](#macos)
    * [Windows](#windows)
    * [Alpine, Arch, and Raspberry Pi OS](#alpine-arch-and-raspberry-pi-os)
* [Cargo Package Manager](#cargo-package-manager-most-platforms)
* [Community Supported Repositories](#community-supported-repositories)
    * [Alpine Linux](#alpine-linux)
    * [Arch Linux](#arch-linux)
    * [Gnu Guix Package Manager](#gnu-guix-package-manager)
    * [FreeBSD](#freebsd)
    * [Windows](#windows-1)

## Official Installation Methods

### Debian and derivatives

Download the `.deb` file from the [releases page][releases] and install with:

    sudo dpkg -i /path/to/git-interactive-rebase-tool-*.deb

The executable will be installed to `/usr/bin`.

### Red Hat Linux, Fedora and derivatives

Download the `.rpm` file from the [releases page][releases] and install with your package manager of choice:

    sudo rpm -i /path/to/git-interactive-rebase-tool-*.rpm
    sudo yum localinstall /path/to/git-interactive-rebase-tool-*.rpm
    sudo dnf localinstall /path/to/git-interactive-rebase-tool-*.rpm

### macOS

#### With [Homebrew](https://brew.sh/)

    brew install git-interactive-rebase-tool

#### Manual install

Download the macOS binary from the [releases page][releases] and copy it as `interactive-rebase-tool` to a location on your `PATH`.

#### Notes

On macOS, Terminal.app does not support highlighting the selected line(s). If you want this feature you will need to use
a terminal emulator like [iTerm2](https://iterm2.com/index.html).

### Windows

*Note: Windows binaries are not fully tested. If you are having issues please report them.*

Download the tool from the [releases page][releases] and save it to a known location.

### Alpine, Arch, and Raspberry Pi OS

#### Manual Install

Download the binary from the [releases page][releases] and copy it as `interactive-rebase-tool` to a location on your `PATH`.

## Cargo Package Manager (Most platforms)

The project can be installed directly from [crates.io](https://crates.io/crates/git-interactive-rebase-tool) via cargo.

    cargo install git-interactive-rebase-tool

## Community Supported Repositories

Community supported repositories are not officially supported, as they are maintained by community members. As such, they do not always provide the latest version. If you run into an issue with a community repository, please reach out to the community member supporting the platform.

### Alpine Linux

Install the [git-interactive-rebase-tool](https://pkgs.alpinelinux.org/packages?name=git-interactive-rebase-tool) package from the community repository (since Alpine v3.14):

    apk add git-interactive-rebase-tool

### Arch Linux

With your [AUR](https://aur.archlinux.org/) helper of choice:

    yay -S git-interactive-rebase-tool

#### Manual Install

1. Download the [package snapshot](https://aur.archlinux.org/packages/git-interactive-rebase-tool/)
2. Extract to a known location
3. Run `makepkg -si` from the extracted location

### Gnu Guix Package Manager

    guix install git-interactive-rebase-tool

#### In a temporary environment

    guix shell git-interactive-rebase-tool

#### In a temporary container (Linux namespace)

    guix shell --container git-interactive-rebase-tool

### FreeBSD

FreeBSD support is provided by the community, and while attempts are made to ensure everything works on the platform, it is not officially supported. If you run into problems please [create an issue](https://github.com/MitMaro/git-interactive-rebase-tool/issues/new) describing the problem.

With [Ports](https://www.freebsd.org/ports/) using `pkg`

    pkg install interactive_rebase_tool

#### Manual

    cd /usr/ports/devel/interactive_rebase_tool && make install clean

### Windows

#### With [Chocolatey](https://chocolatey.org/)

    choco install git-interactive-rebase-tool

#### With [Scoop](https://scoop.sh/)

    scoop install git-interactive-rebase-tool

[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
