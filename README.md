[![Crates.io](https://img.shields.io/crates/v/git-interactive-rebase-tool.svg)][crates-io]
[![Build Status](https://travis-ci.org/MitMaro/git-interactive-rebase-tool.svg?branch=master)][travis-build]
[![Build status](https://ci.appveyor.com/api/projects/status/3a6j6n4o5x6aa763/branch/master?svg=true)][appveyor-build]
[![GitHub license](https://img.shields.io/badge/license-ISC-blue.svg)][license]

# Git Interactive Rebase Tool

Native cross platform full feature terminal based sequence editor for git interactive rebase. Written in Rust using ncurses.

![Image](git-interactive-tool.gif?raw=true)

## Install

#### Debian and derivatives

##### Install

Download the `.deb` file from the [releases page][releases] and install. The executable will be installed to `/usr/bin`.

##### Configure Git

    git config --global sequence.editor interactive-rebase-tool

#### MacOS and OSX

##### Install

Download the `macos-interactive-rebase-tool` from the [releases page][releases] and copy it as
`interactive-rebase-tool` to a location on your `PATH`.

##### Configure Git

    git config --global sequence.editor interactive-rebase-tool

#### Windows

*Note: Windows binaries are not fully tested. If you are having issues please report them.*

##### Install

Download the tool from the [releases page][releases] and save it to a known location. There are builds that use the GNU
toolchain, tagged `gnu`, and builds that use the Microsoft toolchain, tagged `msvc`. There are 32 and 64 bit versions
for each tool chain. If you are unsure which binary to download, you probably want the 64 bit Microsoft toolchain
build.

##### Configure Git

    git config --global core.editor "'C:/path/to/interactive-rebase-tool'"
    
#### From source

##### Install

With Rust's package manager [cargo](https://github.com/rust-lang/cargo), you can install *git-interactive-rebase-tool
* via:

```shell
cargo install git-interactive-rebase-tool
```

##### Configure Git

Cargo adds *git-interactive-rebase-tool* automatically to your PATH, so you can simply run:

    git config --global sequence.editor interactive-rebase-tool

## Usage

```shell
interactive-rebase-tool <rebase-todo-filepath>
interactive-rebase-tool --help
interactive-rebase-tool --version
```

### Getting Help

The tool has built in help that can be accessed by hitting the `?` key.

### Key Bindings

| Key          | Description |
| ------------ | ----------- |
|  Up          | Move selection up |
|  Down        | Move selection down |
|  Page Up     | Move selection up five lines |
|  Page Down   | Move selection down five lines |
|  `q`         | Abort interactive rebase |
|  `Q`         | Immediately abort interactive rebase |
|  `w`         | Write interactive rebase file |
|  `W`         | Immediately write interactive rebase file |
|  `?`         | Show help |
|  `c`         | Show commit information |
|  `j`         | Move selected commit down |
|  `k`         | Move selected commit up |
|  `p`         | Set selected commit to be picked |
|  `r`         | Set selected commit to be reworded |
|  `e`         | Set selected commit to be edited |
|  `s`         | Set selected commit to be squashed |
|  `f`         | Set selected commit to be fixed-up |
|  `d`         | Set selected commit to be dropped |
|  `v`         | View diff at this commit |


## Development

### Install Rust

To start developing the project you will need to [install Rust](https://doc.rust-lang.org/book/getting-started.html),
which can generally be done using [rustup](https://www.rustup.rs/).

### Setup

#### Debian and derivatives

You will need `build-essential` and `libncurses5-dev` to build the project.
Additionally you will need `pkg-config` and `liblzma-dev` if you wish to build
a release. They can be installed using `apt-get`:

    sudo apt-get install build-essential libncurses5-dev
    sudo apt-get install pkg-config liblzma-dev
    sudo apt-get install cmake libssl-dev


### Build and run

Use cargo to build and run the project. From the project project root run:

    # only build
    cargo build --release
    # build and run
    cargo run <path-to-git-rebase-todo-file>


### Release

##### Install Cargo Deb

Cargo Deb has not been released to creates.io so it will need to be installed from the GitHub repository.

    cargo install cargo-deb

##### Building

    cargo build --release
    cargo deb

A deb file will be written to `target/debian/interactive-rebase-tool_*.deb`.

## TODO

 - [ ] Support for `exec` command
 - [ ] Insert commit
 - [ ] Configure key bindings

## Related Projects

* [rebase-editor](https://github.com/sjurba/rebase-editor) is a very similar project but is [not fully cross platform](https://github.com/sjurba/rebase-editor/issues/7) and requires NodeJS to be installed.

## License

Git Interactive Rebase Tool is released under the ISC license. See [LICENSE](LICENSE).

[crates-io]:https://crates.io/crates/git-interactive-rebase-tool
[travis-build]:https://travis-ci.org/MitMaro/git-interactive-rebase-tool
[appveyor-build]:https://ci.appveyor.com/project/MitMaro/git-interactive-rebase-tool/branch/master
[license]:https://raw.githubusercontent.com/MitMaro/git-interactive-rebase-tool/master/LICENSE
[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
