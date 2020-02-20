[![Crates.io](https://img.shields.io/crates/v/git-interactive-rebase-tool.svg)][crates-io]
[![GitHub license](https://img.shields.io/badge/license-GPL-blue.svg)][license]

# Git Interactive Rebase Tool

Native cross-platform full feature terminal based [sequence editor][git-sequence-editor] for interactive rebase in
Git 1.7.8+. Written in Rust using ncurses.

![Git Interactive Rebase Tool](/docs/assets/images/git-interactive-rebase-demo.gif?raw=true)

**This is the documentation for the development build. For the current stable release please use the
[1.2.x documentation](https://github.com/MitMaro/git-interactive-rebase-tool/tree/1.2.1/README.md).**

## Install

* [Cargo](./readme/install.md#cargo-package-manager)
* [Arch](./readme/install.md#arch-linux)
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

#### Standard Command Pompt

    git config --global sequence.editor "'C:/path/to/interactive-rebase-tool.exe'"

#### GitBash

GitBash requires the use of `winpty` in order to work correctly, so to set the editor use:

    git config --global sequence.editor "winpty /c/path/to/interactive-rebase-tool.exe"

#### Notes

Windows before version 10 has [serious rendering issues with saturated darker colors](https://devblogs.microsoft.com/commandline/updating-the-windows-console-colors/),
such as the blue color, that are completely illegible on modern displays. While it is possible to avoid using the
saturated colors, a better option is to update the theme using Microsoft's [ColorTool](https://github.com/Microsoft/Terminal/tree/master/src/tools/ColorTool).

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

#### General

| Key                        | Default | Type   | Description |
| -------------------------- | ------- | ------ | ----------- |
| `autoSelectNext`           | false   | bool   | If true, auto select the next line after action modification |
| `verticalSpacingCharacter` | ~       | String | Vertical spacing character. Can be set to an empty string. |

#### Colors

The valid colors are the [eight original 8 ANSI colors][ANSIColors]. They are `black`, `blue`, `cyan`, `green`,
`magenta`, `red`, `white` and `yellow`. Each terminal controls the exact color for these color names. On terminals that
support 256 colors, a color triplet with the format `<red>,<green>,<blue>` can be used. Each color has a range of 0 to
255 with `255, 255, 255` resulting in white and `0,0,0` resulting in black. A value of `-1` or `transparent` can be used
to use the default terminal color.

| Key                       | Default  | Type  | Description |
| ------------------------- | -------- | ----- | ----------- |
| `breakColor`              | white    | Color | Color used for the break action |
| `diffAddColor`            | green    | Color | Color used for lines and files added in a diff |
| `diffChangeColor`         | yellow   | Color | Color used for lines and files changed in a diff |
| `diffRemoveColor`         | red      | Color | Color used for lines and files removed in a diff |
| `dropColor`               | red      | Color | Color used for the drop action |
| `editColor`               | blue     | Color | Color used for the edit action |
| `fixupColor`              | magenta  | Color | Color used for the fixup action |
| `foregroundColor`         | white    | Color | Color used for most text and the UI |
| `indicatorColor`          | cyan     | Color | Color used for text the indicates or needs to standout  |
| `pickColor`               | green    | Color | Color used for the pick action |
| `rewordColor`             | yellow   | Color | Color used for the reword action |
| `selectedBackgroundColor` | 35,35,40 | Color | Color used as the background color for the selected line |
| `squashColor`             | cyan     | Color | Color used for the squash action |

#### Key Bindings

| Key                        | Default  | Type   | Description |
| -------------------------- | -------- | ------ | ----------- |
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
| `inputMoveDown`            | Down     | String | Key for moving the cursor down |
| `inputMoveLeft`            | Left     | String | Key for moving the cursor left |
| `inputMoveRight`           | Right    | String | Key for moving the cursor right |
| `inputMoveSelectionDown`   | j        | String | Key for moving the selected line(s) down |
| `inputMoveSelectionUp`     | k        | String | Key for moving the selected line(s) up |
| `inputMoveStepDown`        | PageDown | String | Key for moving the cursor down by a large step |
| `inputMoveStepUp`          | PageUp   | String | Key for moving the cursor up  by a large step|
| `inputMoveUp`              | Up       | String | Key for moving the cursor up |
| `inputOpenInExternalEditor`| !        | String | Key for opening the external editor |
| `inputRebase`              | w        | String | Key for rebasing with confirmation |
| `inputShowCommit`          | c        | String | Key for showing the selected commit |
| `inputToggleVisualMode`    | v        | String | Key for toggling visual mode |

##### Special Keys

| Key        | Description |
| ---------- | ----------- |
| `PageDown` | Page Down key |
| `Down`     | Down arrow key |
| `Left`     | Left arrow key |
| `Right`    | Right arrow key |
| `Up`       | Up arrow key |
| `PageUp`   | Page Up key |

#### Configuring External Editor

The external editor action will first attempt to start the editor defined by the
[Git configuration "core.editor"][git-core-editor], followed by the `VISUAL` and
`EDITOR` environment variables. Finally, if neither is set the external editor
defaults to using `vi`.

The `%` character in the value will be replaced with the git rebase todo file.
If the `%` character is not found, then the git rebase todo file will be
provided as the last argument.

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


### Format

Use rust-fmt format project. **You need to run format with nightly.** The current nightly rust version is
nightly-2019-09-13. You can find the current nightly rust version in the `scripts/format.bash`.

    # format code before you commit
    cargo +nightly-2019-09-13 fmt


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
[git-core-editor]:https://www.git-scm.com/book/en/v2/Customizing-Git-Git-Configuration#_code_core_editor_code
[git-sequence-editor]:https://git-scm.com/docs/git-config#git-config-sequenceeditor
[install-rust]:https://doc.rust-lang.org/book/getting-started.html
[license]:https://raw.githubusercontent.com/MitMaro/git-interactive-rebase-tool/master/LICENSE
[rebase-editor-issue-7]:https://github.com/sjurba/rebase-editor/issues/7
[rebase-editor]:https://github.com/sjurba/rebase-editor
[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
[rustup]:https://www.rustup.rs/
[travis-build]:https://travis-ci.org/MitMaro/git-interactive-rebase-tool
