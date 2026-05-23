# Introduction

`open-data-license` is a Rust library crate for representing common open data licenses in dataset metadata.

It provides:

- Stable enum variants for Creative Commons, Open Data Commons, and Linux Foundation CDLA licenses.
- Stable crate-defined identifiers for serde, `Display`, and `FromStr`.
- SPDX identifiers for metadata export.
- Canonical rights URIs for user-facing license links.
- Compatibility helpers for dataset-combination workflows.
- Optional `utoipa` schema derives behind a feature flag.

Supported coverage includes Creative Commons data licenses (`CC0`, `CC BY`, `CC BY-SA`, `CC BY-NC`, `CC BY-NC-SA`), Open Data Commons licenses (`PDDL`, `ODC-BY`, `ODbL`), Community Data License Agreement licenses (`CDLA-Permissive-2.0`, `CDLA-Sharing-1.0`), and Creative Commons Public Domain Mark (`PDM`).

The crate is intended for metadata systems that need to carry license information alongside scientific or research datasets. It is not a substitute for legal review of the underlying license text.

The public API is intentionally forward-compatible. `DataLicense` and `DataUseRestrictionKind` are `#[non_exhaustive]`, so downstream `match` expressions should include a wildcard arm. The `utoipa` integration is also opt-in so consumers that only need metadata helpers do not pull OpenAPI dependencies transitively.
