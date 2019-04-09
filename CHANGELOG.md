# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) 
and this project adheres to [Semantic Versioning](http://semver.org/).

## [1.0.0] - 2019-04-09

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

[Unreleased]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/1.0.0...HEAD
[1.0.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.7.0...1.0.0
[0.7.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.6.0...0.7.0
[0.6.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/MitMaro/git-interactive-rebase-tool/compare/0.1.0...0.2.0
