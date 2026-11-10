# open-data-license vision

A small Rust library for representing open data licenses in scientific and
research dataset metadata. The `DataLicense` enum carries SPDX identifiers,
canonical rights URIs, display names, and compatibility helpers for combining
licensed datasets, with an opt-in `utoipa` schema feature for API surfaces.

## Problem

Dataset metadata has to name a license in several vocabularies at once — a
human display name, an SPDX identifier, and a rights URI — and combination
workflows have to know whether two licenses may be merged. Free-form strings and
per-project enums get one of those wrong, usually silently.

## Principles

- **Metadata, not legal advice.** The crate surfaces the underlying SPDX
  identifier and rights URI so downstream users can review the actual terms.
- **Conservative compatibility.** Public-domain dedications combine with every
  supported license, commercial and non-commercial licenses do not combine, and
  share-alike obligations are family-local, so cross-family combinations such as
  CC BY-SA with ODbL are rejected rather than argued case by case.
- **Forward-compatible public API.** `DataLicense` and
  `DataUseRestrictionKind` are `#[non_exhaustive]`, so adding coverage does not
  break downstream matches.
- **Stable round trips.** Crate-defined identifiers drive serde, `Display`, and
  `FromStr`, and SPDX parsing is available through `from_spdx_id` and
  `TryFrom<&str>`.
- **Optional heavy surfaces.** `utoipa` derives stay behind a feature so
  consumers that only need metadata helpers do not pull OpenAPI dependencies.
- **No unsafe code.** The crate declares `#![forbid(unsafe_code)]`.
- **Invariants are tested, not asserted in prose.** Compatibility properties are
  covered by property-based tests.

## Scope

Creative Commons data licenses (`CC0`, `CC BY`, `CC BY-SA`, `CC BY-NC`,
`CC BY-NC-SA`), Open Data Commons licenses (`PDDL`, `ODC-BY`, `ODbL`), Community
Data License Agreement licenses (`CDLA-Permissive-2.0`, `CDLA-Sharing-1.0`), the
Creative Commons Public Domain Mark (`PDM`), a data-use restriction kind, an
mdBook under `docs/`, and a published crate plus Pages documentation site.

## Non-goals

- It does not interpret license text or decide whether a specific combination is
  lawful.
- It does not cover licenses outside the supported families; `from_spdx_id`
  returns `None` for identifiers it does not map.

## Place in the fleet

canix treats the repository as a documentation/UI helper project: it has a
`PagesSite` entry in the topology and appears in Atlas maintenance and update
automation, while release and prebuild registries deliberately exclude it.
