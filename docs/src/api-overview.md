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

The enum derives serde serialization and deserialization plus `strum` display and parsing.

## Features

- `utoipa`: derives `utoipa::ToSchema` for `DataLicense`, `DataUseRestrictionKind`, and `DataUseRestrictionSpec`.

## Metadata Methods

- `spdx_id()` returns the SPDX license identifier.
- `rights_uri()` returns the canonical URL for the license text.
- `display_name()` returns a human-readable license name.
- `is_public_domain()` reports public-domain dedications.
- `allows_commercial_use()` reports whether the license permits commercial use.
- `requires_share_alike()` reports share-alike/copyleft requirements.
- `requires_attribution()` reports attribution requirements.

## Restriction Metadata

`DataUseRestrictionKind` and `DataUseRestrictionSpec` represent metadata-only use restrictions that can be carried alongside a dataset license.
