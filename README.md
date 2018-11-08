[![Crates.io](https://img.shields.io/crates/v/git-interactive-rebase-tool.svg)][crates-io]
[![Build Status](https://travis-ci.org/MitMaro/git-interactive-rebase-tool.svg?branch=master)][travis-build]
[![Build status](https://ci.appveyor.com/api/projects/status/3a6j6n4o5x6aa763/branch/master?svg=true)][appveyor-build]
[![GitHub license](https://img.shields.io/badge/license-ISC-blue.svg)][license]

# Git Interactive Rebase Tool

Native cross-platform full feature terminal based [sequence editor][git-sequence-editor] for interactive rebase in
Git 1.7.8+. Written in Rust using ncurses.

![Image](git-interactive-tool.gif?raw=true)


## Install

#### Debian and derivatives

##### Install

Download the `.deb` file from the [releases page][releases] and install. The executable will be installed to `/usr/bin`.

You may need to install ncurses with `apt-get install libncurses5` if it is not satisfied.

##### Configure Git

    git config --global sequence.editor interactive-rebase-tool

#### MacOS and OSX

##### Install

Download the `macos-interactive-rebase-tool` from the [releases page][releases] and copy it as `interactive-rebase-tool`
to a location on your `PATH`.

##### Configure Git

    git config --global sequence.editor interactive-rebase-tool

#### Windows

*Note: Windows binaries are not fully tested. If you are having issues please report them.*

##### Install

Download the tool from the [releases page][releases] and save it to a known location. There are 32-bit and 64-bit
versions if you are unsure which binary to download, you probably want the 64-bit build.

##### Configure Git

    git config --global core.editor "'C:/path/to/interactive-rebase-tool'"
    
#### From source

##### Install

With Rust's package manager [cargo], you can install *git-interactive-rebase-tool* via:

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

The tool has built-in help that can be accessed by hitting the `?` key.


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


### Temporary Override

You can temporarily use a different sequence editor by using the `GIT_SEQUENCE_EDITOR` environment variable:

    GIT_SEQUENCE_EDITOR=emacs git rebase -i [<upstream> [<branch>]]


### Configuration

The tool can be configured using the [git config][git-config] command. Invalid values are ignored and the default used
instead. String values are case-insensitive.

#### Example

```
git config --global interactive-rebase-tool.foregroundColor black
```

#### Options

| Key                | Default | Type  | Description |
| ------------------ | ------- | ----- | ----------- |
| `autoSelectNext`   | false   | bool  | If true, auto select the next line after action modification |
| `foregroundColor`  | white   | Color | Color used for most text and the UI |
| `indicatorColor`   | yellow  | Color | Color used for text the indicates or needs to standout  |
| `errorColor`       | red     | Color | Color used for showing error messages  |
| `diffAddColor`     | green   | Color | Color used for lines added in a diff |
| `diffRemoveColor`  | red     | Color | Color used for lines removed in a diff |
| `pickColor`        | green   | Color | Color used for the pick action |
| `rewordColor`      | yellow  | Color | Color used for the reword action |
| `editColor`        | blue    | Color | Color used for the edit action |
| `squashColor`      | cyan    | Color | Color used for the squash action |
| `fixupColor`       | magenta | Color | Color used for the fixup action |
| `dropColor`        | red     | Color | Color used for the drop action |

#### Valid Color Values

The valid colors are the [eight original 8 ANSI colors][ANSIColors]. They are black, blue, cyan, green, magenta, red,
white and yellow. Each terminal controls the exact color for these color names.

## Development

### Install Rust

To start developing the project you will need to [install Rust][install-rust], which can generally be done using
[rustup].


### Setup

#### Debian and derivatives

You will need `build-essential` and `libncurses5-dev` to build the project. Additionally, you will need `pkg-config` and
`liblzma-dev` if you wish to build a release. They can be installed using `apt-get`:

    sudo apt-get install build-essential libncurses5-dev
    sudo apt-get install pkg-config liblzma-dev


### Build and run

Use cargo to build and run the project. From the project root run:

    # only build
    cargo build --release
    # build and run
    cargo run <path-to-git-rebase-todo-file>


### Release

##### Install Cargo Deb

    cargo install cargo-deb

##### Building

    cargo build --release
    cargo deb

A deb file will be written to `target/debian/interactive-rebase-tool_*.deb`.

## TODO

 - [ ] Full support for `exec` action
 - [ ] Insert action
 - [ ] Configure key bindings

## Related Projects

* [rebase-editor] is a very similar project but is [not fully cross-platform][rebase-editor-issue-7] and uses Node.js.

## License

Git Interactive Rebase Tool is released under the ISC license. See [LICENSE](LICENSE).

[ANSIColors]:https://en.wikipedia.org/wiki/ANSI_escape_code#3/4_bit
[appveyor-build]:https://ci.appveyor.com/project/MitMaro/git-interactive-rebase-tool/branch/master
[cargo]:https://github.com/rust-lang/cargo
[crates-io]:https://crates.io/crates/git-interactive-rebase-tool
[git-config]:https://git-scm.com/docs/git-config
[git-sequence-editor]:https://git-scm.com/docs/git-config#git-config-sequenceeditor
[install-rust]:https://doc.rust-lang.org/book/getting-started.html
[license]:https://raw.githubusercontent.com/MitMaro/git-interactive-rebase-tool/master/LICENSE
[rebase-editor-issue-7]:https://github.com/sjurba/rebase-editor/issues/7
[rebase-editor]:https://github.com/sjurba/rebase-editor
[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
[rustup]:https://www.rustup.rs/
[travis-build]:https://travis-ci.org/MitMaro/git-interactive-rebase-tool
