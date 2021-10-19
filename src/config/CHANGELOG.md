# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) and this project adheres to [Semantic Versioning](http://semver.org/).

See the [migration guide](migration-guide.md) for migrating between major versions.

## 2.0.0 - Unreleased

### Added

- Implemented `TryFrom<&Repository>` for `Config`
- Implemented `TryFrom<&Config>` for `Config`, `GitConfig`, `KeyBindings`, and `Theme`
- Implemented `Default` for `Config`, `GitConfig`, `KeyBindings`, and `Theme`

### Changed

- Now only accepts `girt-git` structs instead of direct `git2-rs` structs 
- `GitConfig::new` no longer takes a config reference, and now returns a default instance

### Removed

- `create_config` test utility
- `create_theme` test utility

## 1.0.0 - 2021-07-05

### Added
- Initial release
