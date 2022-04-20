[![Crates.io](https://img.shields.io/crates/v/git-interactive-rebase-tool.svg)][crates-io]
[![GitHub license](https://img.shields.io/badge/license-GPL-blue.svg)][license]
[![Coverage Status](https://coveralls.io/repos/github/MitMaro/git-interactive-rebase-tool/badge.svg?branch=master)](https://coveralls.io/github/MitMaro/git-interactive-rebase-tool?branch=master)

# Git Interactive Rebase Tool

Native cross-platform full feature terminal based [sequence editor][git-sequence-editor] for interactive rebase in Git 1.7.8+.

[![Git Interactive Rebase Tool](/docs/assets/images/girt-demo.gif?raw=true)](https://youtu.be/q3tzb-gQC0w)

**This is the documentation for the development build. For the current stable release, please use the [2.1.x documentation](https://github.com/MitMaro/git-interactive-rebase-tool/tree/2.1.0/README.md).**

## Table of Contents

* [Features](./README.md#features)
* [Install](./readme/install.md)
* [Setup](./README.md#setup)
* [Usage](./README.md#usage)
* [Customization](./readme/customization.md)
* [Development](./README.md#development)
* [Related Projects](./README.md#related-projects)
* [License](./README.md#license)

## Features

### Cross-platform

Built and works on Linux, macOS, Windows and many others.

### Set action

Easily set the action to `pick`, `squash`, `fixup`, `edit`, `reword` and `drop`.

![Basic operations](/docs/assets/images/girt-set-actions.gif?raw=true)

### Reorder rebase list

Reorder the action list with a single key press.

![Reorder items](/docs/assets/images/girt-reorder.gif?raw=true)

### Multiline modification

Change action and reorder multiple lines at once with visual mode.

![Visual mode](/docs/assets/images/girt-visual-mode.gif?raw=true)

### Toggle `break`s

![Toggle breaks](/docs/assets/images/girt-break.gif?raw=true)

### View commit details and diff

View the commit overview, and a full commit diff with a press of a key.

![Commit overview](/docs/assets/images/girt-commit-overview.gif?raw=true)

![Commit diff](/docs/assets/images/girt-commit-diff.gif?raw=true)

### Unicode and Emoji support

![Unicode support](/docs/assets/images/girt-unicode.png?raw=true)

![Emoji support](/docs/assets/images/girt-emoji.png?raw=true)

### Edit `exec` command

Easily edit the command that is run by an `exec` command.

![exec action command edit](/docs/assets/images/girt-edit.gif?raw=true)

### Edit in external editor

Need to do something in your Git editor? Quickly shell out to your editor, make a change and return to the tool.

![Shell out to editor](/docs/assets/images/girt-external-editor.gif?raw=true)

## Setup

### Most systems

    git config --global sequence.editor interactive-rebase-tool

### Windows

#### Standard Command Prompt

    git config --global sequence.editor "'C:/path/to/interactive-rebase-tool.exe'"

#### GitBash

GitBash requires the use of `winpty` in order to work correctly, so to set the editor use:

    git config --global sequence.editor "winpty /c/path/to/interactive-rebase-tool.exe"

#### Notes

Windows before version 10 has [serious rendering issues with saturated darker colors](https://devblogs.microsoft.com/commandline/updating-the-windows-console-colors/), such as the blue color that is entirely illegible on modern displays. While it is possible to avoid using saturated colors, a better option is to update the theme using Microsoft's [ColorTool](https://github.com/Microsoft/Terminal/tree/master/src/tools/ColorTool).


### Temporary Override

You can temporarily use a different sequence editor by using the `GIT_SEQUENCE_EDITOR` environment variable:

    GIT_SEQUENCE_EDITOR=emacs git rebase -i [<upstream> [<branch>]]

## Usage

```shell
interactive-rebase-tool <rebase-todo-filepath>
interactive-rebase-tool --help
interactive-rebase-tool --version
```

### Getting Help

The tool has built-in help that can be accessed by hitting the `?` key.

### Default Key Bindings

Key bindings can be configured, see [configuration](readme/customization.md#key-bindings) for more information.

| Key         | Mode   | Description                               |
|-------------| ------ |-------------------------------------------|
| Up          | All    | Move selection up                         |
| Down        | All    | Move selection down                       |
| Page Up     | All    | Move selection up five lines              |
| Page Down   | All    | Move selection down five lines            |
| Home        | All    | Move selection to start of list           |
| End         | All    | Move selection to home of list            |
| `q`         | Normal | Abort interactive rebase                  |
| `Q`         | Normal | Immediately abort interactive rebase      |
| `w`         | Normal | Write interactive rebase file             |
| `W`         | Normal | Immediately write interactive rebase file |
| `?`         | All    | Show help                                 |
| `c`         | Normal | Show commit information                   |
| `j`         | All    | Move selected commit(s) down              |
| `k`         | All    | Move selected commit(s) up                |
| `b`         | Normal | Toggle break action                       |
| `p`         | All    | Set selected commit(s) to be picked       |
| `r`         | All    | Set selected commit(s) to be reworded     |
| `e`         | All    | Set selected commit(s) to be edited       |
| `s`         | All    | Set selected commit(s) to be squashed     |
| `f`         | All    | Set selected commit(s) to be fixed-up     |
| `d`         | All    | Set selected commit(s) to be dropped      |
| `E`         | Normal | Edit the command of an exec action        |
| `v`         | All    | Enter and exit visual mode                |
| `I`         | Normal | Insert a new line                         |
| `Delete`    | All    | Remove selected lines                     |
| `Control+z` | All    | Undo the previous change                  |
| `Control+y` | All    | Redo the previously undone change         |
| Down        | Diff   | Scroll view down                          |
| Up          | Diff   | Scroll view up                            |
| Left        | Diff   | Scroll view left                          |
| Right       | Diff   | Scroll view right                         |
| Home        | Diff   | Scroll view to the top                    |
| End         | Diff   | Scroll view to the end                    |
| PageUp      | Diff   | Scroll view a step up                     |
| PageDown    | Diff   | Scroll view a step down                   |
| `d`         | Diff   | Show full commit diff                     |

## Development

### Install Rust

To start developing the project, you will need to [install Rust][install-rust], which can generally be done using [rustup].

### Setup

#### Cargo Make

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) as a task runner. To install:

    cargo install --force cargo-make

#### Debian and derivatives

If you plan to build a release package you will need `pkg-config` and `liblzma-dev`. They can be installed using `apt`:

    sudo apt install pkg-config liblzma-dev

### Build and run

To build or run the project, from the project root run:

    # only build
    cargo make build --release
    # build and run
    cargo run -- <path-to-git-rebase-todo-file>

Sample rebase todo files can be found in ./test/fixtures.

### Tests

Automated tests are available for all features and can be run with:

    cargo make test

### Docs

API docs for the project are generated using Rust Doc:

    cargo make docs

### Linting

An addition to the lints provided by rustc, this project uses Clippy to provide additional linting, run with:

    cargo make lint

This will run lints using stable and nightly. The nightly lints may show errors, but will not result in a failure.

### Format

This project uses rust-fmt to provide a consistent format. A helpful script will ensure that all files are formatted correctly:

    cargo make format

### Coverage

The project use [Tarpaulin](https://github.com/xd009642/tarpaulin) to generate coverage reports. Coverage reports are used to find gaps in tests. To generate the coverage report:

    cargo make coverage

An addition to the report printed to the CLI, an HTML report can be found in the `coverage` directory.

### Release

##### Building

    cargo make deb

A deb file will be written to `target/debian/interactive-rebase-tool_*.deb`.

## Related Projects

* [rebase-editor](https://github.com/sjurba/rebase-editor) is a very similar project written in Node.js.
* [cj-git-patchtool](https://github.com/pflanze/cj-git-patchtool) is another tool for doing git rebase en masse.

## License

Git Interactive Rebase Tool is released under the GPLv3 license. See [LICENSE](LICENSE).

See [Third Party Licenses](https://gitrebasetool.mitmaro.ca/licenses.html) for licenses of the third-party libraries used by this project.

[appveyor-build]:https://ci.appveyor.com/project/MitMaro/git-interactive-rebase-tool/branch/master
[cargo]:https://github.com/rust-lang/cargo
[crates-io]:https://crates.io/crates/git-interactive-rebase-tool
[git-sequence-editor]:https://git-scm.com/docs/git-config#git-config-sequenceeditor
[install-rust]:https://doc.rust-lang.org/book/getting-started.html
[license]:https://raw.githubusercontent.com/MitMaro/git-interactive-rebase-tool/master/LICENSE
[releases]:https://github.com/MitMaro/git-interactive-rebase-tool/releases
[rustup]:https://www.rustup.rs/
[travis-build]:https://travis-ci.org/MitMaro/git-interactive-rebase-tool
