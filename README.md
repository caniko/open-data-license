# open-data-license

[![crates.io](https://img.shields.io/crates/v/open-data-license)](https://crates.io/crates/open-data-license)
[![docs.rs](https://img.shields.io/docsrs/open-data-license)](https://docs.rs/open-data-license)
[![license](https://img.shields.io/crates/l/open-data-license)](https://github.com/caniko/open-data-license/blob/trunk/LICENSE)
[![CI](https://github.com/caniko/open-data-license/actions/workflows/ci.yaml/badge.svg?branch=trunk)](https://github.com/caniko/open-data-license/actions)

`open-data-license` provides a small Rust enum for open data licenses used in scientific and research datasets. It exposes SPDX identifiers, canonical rights URIs, display names, and compatibility helpers for combining licensed datasets.

The crate currently covers Creative Commons data licenses (`CC0`, `CC BY`, `CC BY-SA`, `CC BY-NC`, `CC BY-NC-SA`), Open Data Commons licenses (`PDDL`, `ODC-BY`, `ODbL`), Linux Foundation Community Data License Agreement licenses (`CDLA-Permissive-2.0`, `CDLA-Sharing-1.0`), and Creative Commons Public Domain Mark (`PDM`).

## Install

When using a published crates.io release:

```toml
[dependencies]
open-data-license = "0.2"
```

Enable the optional `utoipa` feature when deriving OpenAPI schemas for the crate types:

```toml
[dependencies]
open-data-license = { version = "0.2", features = ["utoipa"] }
```

If you need repository state that is newer than the last published release, depend on a Git revision or a known tag:

```toml
[dependencies]
open-data-license = { git = "https://github.com/caniko/open-data-license.git", tag = "0.2.0" }
```

## Example

```rust
use open_data_license::DataLicense;

let license = DataLicense::CcBySa;

assert_eq!(license.spdx_id(), "CC-BY-SA-4.0");
assert!(license.requires_attribution());
assert!(license.requires_share_alike());
assert!(license.is_compatible_with(&DataLicense::CcBy));
```

## Compatibility Model

The compatibility helpers provide metadata-level checks for dataset combination workflows. Public-domain dedications are compatible with every supported license. Commercial and non-commercial licenses are incompatible. Share-alike obligations are family-local, so cross-family combinations such as `CC-BY-SA-4.0` with `ODbL-1.0` are rejected conservatively.

This crate does not provide legal advice. Consumers should surface the underlying SPDX identifier and rights URI so downstream users can review the actual license terms.

## Documentation

- API documentation: <https://docs.rs/open-data-license>
- Source repository: <https://github.com/caniko/open-data-license>
- Project documentation: <https://caniko.codeberg.page/open-data-license/>

## Release Validation

The strict release gate is:

```sh
nix flake check --keep-going --print-build-logs
nix develop -c cargo publish --dry-run
```

Forgejo Actions for this repository run on the self-hosted `atlas` runner. Codeberg repository Actions must be enabled in Settings -> Units before the workflows can execute.
