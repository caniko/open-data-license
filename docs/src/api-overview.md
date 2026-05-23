# API Overview

## `DataLicense`

`DataLicense` is the primary enum. It supports:

- `Cc0`
- `CcBy`
- `CcBySa`
- `CcByNc`
- `CcByNcSa`
- `Pddl`
- `OdcBy`
- `OdcOdbl`
- `CdlaPermissive2_0`
- `CdlaSharing1_0`
- `Pdm`

The enum derives serde serialization/deserialization and `strum` display, with stable `FromStr` parsing for the same crate-defined identifiers. It is marked `#[non_exhaustive]`, so downstream matches should include a wildcard arm.

## Features

- `utoipa`: derives `utoipa::ToSchema` for `DataLicense`, `DataUseRestrictionKind`, and `DataUseRestrictionSpec`.

## Metadata Methods

- `spdx_id()` returns the SPDX license identifier.
- `from_spdx_id()` parses a canonical SPDX-style identifier into a `DataLicense`.
- `rights_uri()` returns the canonical URL for the license text.
- `display_name()` returns a human-readable license name.
- `is_public_domain()` reports public-domain dedications.
- `allows_commercial_use()` reports whether the license permits commercial use.
- `requires_share_alike()` reports share-alike/copyleft requirements.
- `requires_attribution()` reports attribution requirements.

### Parsing SPDX identifiers

Use `DataLicense::from_spdx_id()` for external metadata values such as `CC-BY-SA-4.0`:

```rust
use open_data_license::DataLicense;

assert_eq!(
    DataLicense::from_spdx_id("CC-BY-SA-4.0"),
    Some(DataLicense::CcBySa)
);
assert_eq!(DataLicense::from_spdx_id("cc-by-sa-4.0"), None);
```

Parsing is case-sensitive. `TryFrom<&str>` is also implemented and returns `UnknownSpdxId` with the original input when the identifier is not recognized.

`Display` and `FromStr` intentionally keep using the crate's stable enum identifiers such as `CC_BY_SA`, preserving their round-trip behavior. Use `spdx_id()` when serializing the external SPDX-style form.

`PDM` is a special case. The crate exposes `PDM-1.0` for symmetry with `spdx_id()` and `from_spdx_id()`, but Public Domain Mark is not an SPDX-listed software license identifier and should be treated as a metadata convenience rather than an official SPDX token.

## Restriction Metadata

`DataUseRestrictionKind` and `DataUseRestrictionSpec` represent metadata-only use restrictions that can be carried alongside a dataset license.

Both types are marked `#[non_exhaustive]` so new restriction categories and fields can be added without requiring a major-version bump.

## Consumer Guidance

- Use `Display`, `FromStr`, and serde when persisting the crate's stable internal identifiers such as `CC_BY` or `ODC_ODBL`.
- Use `spdx_id()` and `from_spdx_id()` when exchanging values with external metadata systems that already speak SPDX-style license strings.
- Expect new enum variants over time. Avoid exhaustive pattern matches and avoid assuming the current license set is closed.
