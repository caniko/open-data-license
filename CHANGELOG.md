# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-05-23

### Added

- Added optional `utoipa` schema support behind the `utoipa` feature.
- Added CDLA Permissive 2.0, CDLA Sharing 1.0, and Creative Commons Public Domain Mark coverage.
- Added SPDX identifier parsing through `DataLicense::from_spdx_id` and `TryFrom<&str>`.
- Added property-based tests for compatibility invariants.

### Changed

- Relaxed dependency version requirements so the crate is friendlier as a library dependency.
- Marked public license and restriction types as non-exhaustive for future license coverage.
- Tightened compatibility checks so share-alike licenses are evaluated within license families.
- Preserved `Display` and `FromStr` round-tripping through the crate's stable enum identifiers.

### Fixed

- Fixed cross-family share-alike combinations such as CC BY-SA with ODbL being reported as compatible.
- Added a no-unsafe guarantee with `#![forbid(unsafe_code)]`.

## [0.1.0] - 2026-05-21

### Added

- Initial standalone extraction of the open data license enum, metadata accessors, serde support, and compatibility helpers.
