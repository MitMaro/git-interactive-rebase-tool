[![Crates.io](https://img.shields.io/crates/v/git-interactive-rebase-tool.svg)][![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FMitMaro%2Fgit-interactive-rebase-tool.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2FMitMaro%2Fgit-interactive-rebase-tool?ref=badge_shield)
[crates-io]
[![Build Status](https://travis-ci.org/MitMaro/git-interactive-rebase-tool.svg?branch=master)][travis-build]
[![Build status](https://ci.appveyor.com/api/projects/status/3a6j6n4o5x6aa763/branch/master?svg=true)][appveyor-build]
[![GitHub license](https://img.shields.io/badge/license-GPL-blue.svg)][license]

# Git Interactive Rebase Tool

Native cross-platform full feature terminal based [sequence editor][git-sequence-editor] for interactive rebase in
Git 1.7.8+. Written in Rust using ncurses.

![Git Interactive Rebase Tool](/docs/assets/images/git-interactive-rebase-demo.gif?raw=true)

## Install

* [Cargo](./readme/install.md#cargo-package-manager)
* [Debian](./readme/install.md#debian-and-derivatives)
* [FreeBSD](./readme/install.md#freebsd)
* MacOS
  * [Homebrew](./readme/install.md#macos-via-homebrew)
  * [Manual](./readme/install.md#macos-manual-install)
* [Windows](./readme/install.md#windows)

## Configure

### Most systems

    git config --global sequence.editor interactive-rebase-tool

### Windows

    git config --global sequence.editor "'C:/path/to/interactive-rebase-tool'"

## Usage

```shell
interactive-rebase-tool <rebase-todo-filepath>
interactive-rebase-tool --help
interactive-rebase-tool --version
```

### Getting Help

The tool has built-in help that can be accessed by hitting the `?` key.


### Default Key Bindings

Key bindings can be configured, see [configuration](#configuration) for more information.

| Key          | Mode   | Description |
| ------------ | ------ | ----------- |
|  Up          | All    | Move selection up |
|  Down        | All    | Move selection down |
|  Page Up     | All    | Move selection up five lines |
|  Page Down   | All    | Move selection down five lines |
|  `q`         | Normal | Abort interactive rebase |
|  `Q`         | Normal | Immediately abort interactive rebase |
|  `w`         | Normal | Write interactive rebase file |
|  `W`         | Normal | Immediately write interactive rebase file |
|  `?`         | All    | Show help |
|  `c`         | Normal | Show commit information |
|  `j`         | All    | Move selected commit(s) down |
|  `k`         | All    | Move selected commit(s) up |
|  `b`         | Normal | Toggle break action |
|  `p`         | All    | Set selected commit(s) to be picked |
|  `r`         | All    | Set selected commit(s) to be reworded |
|  `e`         | All    | Set selected commit(s) to be edited |
|  `s`         | All    | Set selected commit(s) to be squashed |
|  `f`         | All    | Set selected commit(s) to be fixed-up |
|  `d`         | All    | Set selected commit(s) to be dropped |
|  `E`         | Normal | Edit the command of an exec action |
|  `V`         | All    | Enter and exit visual mode |


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

| Key                        | Default  | Type   | Description |
| -------------------------- | -------- | ------ | ----------- |
| `autoSelectNext`           | false    | bool   | If true, auto select the next line after action modification |
| `breakColor`               | white    | Color  | Color used for the break action |
| `diffAddColor`             | green    | Color  | Color used for lines and files added in a diff |
| `diffChangeColor`          | yellow   | Color  | Color used for lines and files changed in a diff |
| `diffRemoveColor`          | red      | Color  | Color used for lines and files removed in a diff |
| `dropColor`                | red      | Color  | Color used for the drop action |
| `editColor`                | blue     | Color  | Color used for the edit action |
| `errorColor`               | red      | Color  | Color used for showing error messages  |
| `fixupColor`               | magenta  | Color  | Color used for the fixup action |
| `foregroundColor`          | white    | Color  | Color used for most text and the UI |
| `indicatorColor`           | cyan     | Color  | Color used for text the indicates or needs to standout  |
| `inputAbort`               | q        | String | Key for abort rebase with prompt |
| `inputActionBreak`         | b        | String | Key for setting action to rebase |
| `inputActionDrop`          | d        | String | Key for setting action to drop |
| `inputActionEdit`          | e        | String | Key for setting action to edit |
| `inputActionFixup`         | f        | String | Key for setting action to fixup |
| `inputActionPick`          | p        | String | Key for setting action to pick |
| `inputActionReword`        | r        | String | Key for setting action to reword |
| `inputActionSquash`        | s        | String | Key for setting action to squash |
| `inputConfirmNo`           | n        | String | Key for rejecting a confirmation |
| `inputConfirmYes`          | y        | String | Key for confirming a confirmation |
| `inputEdit`                | E        | String | Key for entering edit mode |
| `inputForceAbort`          | Q        | String | Key for forcing an abort of the rebase |
| `inputForceRebase`         | W        | String | Key for forcing a rebase |
| `inputHelp`                | ?        | String | Key for showing the help |
| `inputMoveDownStep`        | PageDown | String | Key for moving the cursor down by a large step |
| `inputMoveDown`            | Down     | String | Key for moving the cursor down |
| `inputMoveLeft`            | Left     | String | Key for moving the cursor left |
| `inputMoveRight`           | Right    | String | Key for moving the cursor right |
| `inputMoveSelectionDown`   | j        | String | Key for moving the selected line(s) down |
| `inputMoveSelectionUp`     | k        | String | Key for moving the selected line(s) up |
| `inputMoveUp`              | Up       | String | Key for moving the cursor up |
| `inputMoveUpStep`          | PageUp   | String | Key for moving the cursor up  by a large step|
| `inputOpenInExternalEditor`| !        | String | Key for opening the external editor |
| `inputRebase`              | w        | String | Key for rebasing with confirmation |
| `inputShowCommit`          | c        | String | Key for showing the selected commit |
| `inputToggleVisualMode`    | v        | String | Key for toggling visual mode |
| `pickColor`                | green    | Color  | Color used for the pick action |
| `rewordColor`              | yellow   | Color  | Color used for the reword action |
| `squashColor`              | cyan     | Color  | Color used for the squash action |
| `verticalSpacingCharacter` | ~        | String | Vertical spacing character. Can be set to an empty string. |

#### Special Keys

| Key        | Description |
| ---------- | ----------- |
| `PageDown` | Page Down key |
| `Down`     | Down arrow key |
| `Left`     | Left arrow key |
| `Right`    | Right arrow key |
| `Up`       | Up arrow key |
| `PageUp`   | Page Up key |

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

    sudo apt-get install build-essential libncursesw5-dev
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

 - [x] Full support for `exec` action
 - [ ] Insert action
 - [x] Configure key bindings

## Related Projects

* [rebase-editor] is a very similar project written in Node.js.

## License

Git Interactive Rebase Tool is released under the GPLv3 license. See [LICENSE](LICENSE).

See [Third Party Licenses](THIRD_PARTY_LICENSES) for licenses for third-party libraries used by this project.

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


[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FMitMaro%2Fgit-interactive-rebase-tool.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FMitMaro%2Fgit-interactive-rebase-tool?ref=badge_large)