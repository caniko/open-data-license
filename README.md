# open-data-license

`open-data-license` provides a small Rust enum for open data licenses used in scientific and research datasets. It exposes SPDX identifiers, canonical rights URIs, display names, and compatibility helpers for combining licensed datasets.

The crate currently covers Creative Commons data licenses (`CC0`, `CC BY`, `CC BY-SA`, `CC BY-NC`, `CC BY-NC-SA`) and Open Data Commons licenses (`PDDL`, `ODC-BY`, `ODbL`).

## Install

After publication to crates.io:

```toml
[dependencies]
open-data-license = "0.1.0"
```

Enable the optional `utoipa` feature when deriving OpenAPI schemas for the crate types:

```toml
[dependencies]
open-data-license = { version = "0.2", features = ["utoipa"] }
```

Until the crates.io release is published, depend on the tagged Codeberg release:

```toml
[dependencies]
open-data-license = { git = "https://codeberg.org/caniko/open-data-license.git", tag = "v0.1.0" }
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

The compatibility helpers provide metadata-level checks for dataset combination workflows. Public-domain dedications are compatible with every supported license. Non-commercial and commercial-use licenses are treated as incompatible. Share-alike licenses are treated as compatible with licenses in the same restrictiveness band or less restrictive non-share-alike licenses.

This crate does not provide legal advice. Consumers should surface the underlying SPDX identifier and rights URI so downstream users can review the actual license terms.

## Documentation

- API documentation: <https://docs.rs/open-data-license>
- Source repository: <https://codeberg.org/caniko/open-data-license>
- Project documentation: <https://caniko.codeberg.page/open-data-license/>

## Release Validation

The strict release gate is:

```sh
nix flake check --keep-going --print-build-logs
nix develop -c cargo publish --dry-run
```

Forgejo Actions for this repository run on the self-hosted `atlas` runner. Codeberg repository Actions must be enabled in Settings -> Units before the workflows can execute.
