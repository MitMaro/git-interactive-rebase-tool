# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/).

## [Unreleased]
### Added
- Post modified line exec command ([#888](https://github.com/MitMaro/git-interactive-rebase-tool/pull/888))


## [2.3.0] - 2023-07-19
### Added
- Support for update-ref action ([#801](https://github.com/MitMaro/git-interactive-rebase-tool/pull/801))
- Search in the list view ([#797](https://github.com/MitMaro/git-interactive-rebase-tool/pull/797))

## [2.2.1] - 2022-10-13
### Fixed
- Crash on multibyte strings in commit diff view ([#755](https://github.com/MitMaro/git-interactive-rebase-tool/pull/755))

## [2.2.0] - 2022-04-21
### Added
- Added mew keybindings for customizing the scrolling the view. ([#647](https://github.com/MitMaro/git-interactive-rebase-tool/pull/647))
- Multiple non-specific performance improvements.

### Removed
- `ctrl-d` keybinding, since it conflicts with the `ctrl-d` keybinding in Vim. ([#648](https://github.com/MitMaro/git-interactive-rebase-tool/pull/648))

## [2.1.0] - 2021-04-20

### Added
- Label and description to commit edit ([#429](https://github.com/MitMaro/git-interactive-rebase-tool/pull/429))
- Basic support to merge rebasing ([#434](https://github.com/MitMaro/git-interactive-rebase-tool/pull/434))
- Modifier keys can now be provided in any order ([#435](https://github.com/MitMaro/git-interactive-rebase-tool/pull/435))
- Undo and redo changes to the todo list ([#436](https://github.com/MitMaro/git-interactive-rebase-tool/pull/436))
- Support for multiple key bindings per configuration ([#437](https://github.com/MitMaro/git-interactive-rebase-tool/pull/437))
- Open external editor from visual mode ([#442](https://github.com/MitMaro/git-interactive-rebase-tool/pull/442))
- Delete selected lines from the todo list ([#443](https://github.com/MitMaro/git-interactive-rebase-tool/pull/443))
- Insert new exec, commit, label, reset or merge line ([#454](https://github.com/MitMaro/git-interactive-rebase-tool/pull/454), [#458](https://github.com/MitMaro/git-interactive-rebase-tool/pull/458))
- Support home and end in list view ([#455](https://github.com/MitMaro/git-interactive-rebase-tool/pull/455))

### Fixed
- Most modifier key combinations could not be used as key bindings ([#435](https://github.com/MitMaro/git-interactive-rebase-tool/pull/435))
- Several index overflows when modifying the todo list with an external editor ([#441](https://github.com/MitMaro/git-interactive-rebase-tool/pull/441), [#440](https://github.com/MitMaro/git-interactive-rebase-tool/pull/440))

## [2.0.0] - 2021-01-28

### Added
- A diff view to show commit ([#262](https://github.com/MitMaro/git-interactive-rebase-tool/pull/262))
- Page up and down to help view ([#258](https://github.com/MitMaro/git-interactive-rebase-tool/pull/258))
- Page up and down to show commit ([#258](https://github.com/MitMaro/git-interactive-rebase-tool/pull/258))
- Most missing key bindings for special keys ([#239](https://github.com/MitMaro/git-interactive-rebase-tool/pull/239))
- Builtin help for show commit ([#258](https://github.com/MitMaro/git-interactive-rebase-tool/pull/258))
- Number of files change in show commit ([#258](https://github.com/MitMaro/git-interactive-rebase-tool/pull/258))
- Number of total additions and deletions in show commit ([#258](https://github.com/MitMaro/git-interactive-rebase-tool/pull/258))
- The Git "diff.renames" and "diff.rename_limit" options are now respected during show commit ([#258](https://github.com/MitMaro/git-interactive-rebase-tool/pull/258))
- End and Home support during edit ([#309](https://github.com/MitMaro/git-interactive-rebase-tool/pull/309))
- Optional rollback on error or invalid file during external edit ([#329](https://github.com/MitMaro/git-interactive-rebase-tool/pull/329))
- True color support on macOS ([#417](https://github.com/MitMaro/git-interactive-rebase-tool/pull/417))

### Changed
- Replace Curses with Crossterm for input and output ([#415](https://github.com/MitMaro/git-interactive-rebase-tool/pull/415))
- Change page up and page down to scroll half the height of the view area ([#230](https://github.com/MitMaro/git-interactive-rebase-tool/pull/230))
- Improved error handling for executing external editor ([#329](https://github.com/MitMaro/git-interactive-rebase-tool/pull/329))

### Fixed
- Scroll position resetting on resize ([#261](https://github.com/MitMaro/git-interactive-rebase-tool/pull/261))
- Unable to move edit cursor when `inputMoveLeft` or `inputMoveRight` were set to alphanumeric characters ([#309](https://github.com/MitMaro/git-interactive-rebase-tool/pull/309))

### Removed
- Available actions footer from list and show commit views ([#330](https://github.com/MitMaro/git-interactive-rebase-tool/pull/330))

## [1.2.1] - 2020-01-26

### Fixed
- ANSI color support broken on MacOS ([#219](https://github.com/MitMaro/git-interactive-rebase-tool/issues/219)) 

## [1.2.0] - 2020-01-11

### Added
- Support for 256-color terminals
- Highlight of selected line(s) on 256-color terminals ([#148](https://github.com/MitMaro/git-interactive-rebase-tool/issues/148)
- Full support for external editor ([#60](https://github.com/MitMaro/git-interactive-rebase-tool/issues/60))

### Fixed
- Missing ncursesw dependency listing for deb build ([#170](https://github.com/MitMaro/git-interactive-rebase-tool/issues/170))
- Performance issue with show commit ([#167](https://github.com/MitMaro/git-interactive-rebase-tool/issues/167))
- Visual mode index error when changing action or swapping lines ([195](https://github.com/MitMaro/git-interactive-rebase-tool/issues/195)))
- Fixed crash with scrolling to max length
- A empty rebase file now returns a zero exit code ([#197](https://github.com/MitMaro/git-interactive-rebase-tool/issues/197))
- External editing loop when an external editor returns an empty file ([#196](https://github.com/MitMaro/git-interactive-rebase-tool/issues/196))

### Removed
- Unused `errorColor` configuration ([#168](https://github.com/MitMaro/git-interactive-rebase-tool/pull/168))

## [1.1.0] - 2019-08-15

### Added
- Add support for the break action
- The command of an exec action can now be edited
- Visual mode - change action and reorder with multiple selections
- Configuration option for vertical spacing character
- Configurable key bindings
- Horizontal scrolling

### Fixed
- A noop rebase will no longer return a non-zero status code

## [1.0.0] - 2019-04-10

### Added
- Support for unicode characters
- Horizontal and vertical overflow support

### Changed
- Show git now uses libgit2 instead of an external command

## [0.7.0] - 2018-10-28

### Added
- Support git `core.commentChar` option
- Configuration of colors
- Support for the exec action
- Auto-select next line configuration
- Prepend application name to error messages

### Fixed
- Windows creating a new window on run (hopefully)
- Resize not handled in all cases

## [0.6.0] - 2018-02-08

### Added
- man page

## [0.5.0] - 2017-12-29

### Added
- `--version` and `-v` options to print current version

## [0.4.0] - 2017-02-11

### Added
- Support for scrolling
- Support for page up and page down

### Fixed
- Crash on noop rebases

## [0.3.0] - 2017-01-21
### Changed
- Cleaned up help

### Added
- Build setup
- Documentation with `README.md`
- LICENSE

### Changed
- `Cargo.toml` cleaned up

## [0.2.0] - 2017-01-07
### Added
- `Q` key that immediately aborts
- `W` key that immediately resumes rebase

### Fixed
- Fixed actions for `j` and `k` keys

### Changed
- Complete rewrite of the project
- Removed `exec` support
- Friendlier selection indicator

## 0.1.0 - 2016-12-22
### Added
- Initial project release

[Unreleased]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/2.3.0...HEAD
[2.3.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/2.2.1...2.3.0
[2.2.1]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/2.2.0...2.2.1
[2.2.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/2.1.0...2.2.0
[2.1.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/2.0.0...2.1.0
[2.0.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/1.2.1...2.0.0
[1.2.1]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/1.2.0...1.2.1
[1.2.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/1.1.0...1.2.0
[1.1.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/1.0.0...1.1.0
[1.0.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.7.0...1.0.0
[0.7.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.1.0...0.2.0
