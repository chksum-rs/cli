# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `unstable` feature.

## [0.3.1] - 2023-08-19

### Fixed

- Fixed `stdin` option to conflicts only with `paths` arg.
- Fixed force binary name to `chksum`.

### Changed

- Changed usage from `name` to `value_name` for `paths` arg.

### Removed

- Removed `author` from command.

## [0.3.0] - 2023-08-18

### Added

- Added `Target` enum to better handle output printing.
- Added `#![forbid(unsafe_code)]` to forbid unsafe code.
- Added colored output.

### Fixed

- Fixed wrong link in `CHANGELOG.md`.
- Fixed missing stdin examples in `README.md` and `.cargo/README.md`.
- Fixed blank error with new version of `chksum` dependency.

### Changed

- Changed `cargo tarpaulin` command to use `--engine llvm` in GitHub Actions.
- Changed output format.
- Changed algorithms logic (can enabled or disabled via features).

### Removed

- Removed unused import.

## [0.2.0] - 2023-08-13

### Added

- Initial release.

[Unreleased]: https://github.com/ferric-bytes/chksum-cli/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/ferric-bytes/chksum-cli/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/ferric-bytes/chksum-cli/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ferric-bytes/chksum-cli/releases/tag/v0.2.0
