# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add last_used field for applications and clients (introduced in Gotify 2.4.0)

### Changed

- **BREAKING**: Mark all returned models as [`#[non_exhaustive]`](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute). This helps avoiding backwards incompatible changes when new fields are added to Gotify's API.

## [0.3.0] - 2023-09-01

### Changed

- **BREAKING**: Don't create features for optional dependencies
- **BREAKING**: Make TLS backend configurable via feature flags

## [0.2.0] - 2023-08-28

### Changed

- **BREAKING**: Rename `Client::message_stream` to `Client::stream_messages`
- Rework internal request builder
- Improve docs

## [0.1.0] - 2023-08-27

### Added

- Initial release

[Unreleased]: https://github.com/d-k-bo/gotify-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/d-k-bo/gotify-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/d-k-bo/gotify-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/d-k-bo/gotify-rs/releases/tag/v0.0.1
