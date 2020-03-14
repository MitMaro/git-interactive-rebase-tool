# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) 
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - 2020-03-14

### Added
- Added page up and down to help view
- Added page up and down to show commit

### Changed
- Change page up and page down to scroll half the height of the view area

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
- Unused `errorColor` configuration

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
- Show git now uses libgit2 instead of external command

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

[Unreleased]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/1.2.1...HEAD
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
