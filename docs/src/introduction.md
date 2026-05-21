# Introduction

`open-data-license` is a Rust library crate for representing common open data licenses in dataset metadata.

It provides:

- Stable enum variants for Creative Commons and Open Data Commons licenses.
- SPDX identifiers for metadata export.
- Canonical rights URIs for user-facing license links.
- Compatibility helpers for dataset-combination workflows.

The crate is intended for metadata systems that need to carry license information alongside scientific or research datasets. It is not a substitute for legal review of the underlying license text.
