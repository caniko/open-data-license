# Getting Started

When using a published crates.io release, depend on the current crate series:

```toml
[dependencies]
open-data-license = "0.2"
```

Enable the optional `utoipa` feature only when you need OpenAPI schema derives:

```toml
[dependencies]
open-data-license = { version = "0.2", features = ["utoipa"] }
```

If you need repository state that is newer than the last published release, depend on a Git revision or a known tag. The latest tag in this repository today is `v0.1.0`:

```toml
[dependencies]
open-data-license = { git = "https://github.com/caniko/open-data-license.git", tag = "v0.1.0" }
```

Basic usage:

```rust
use open_data_license::DataLicense;

let license = DataLicense::CcBy;

assert_eq!(license.spdx_id(), "CC-BY-4.0");
assert_eq!(
    DataLicense::from_spdx_id("CC-BY-4.0"),
    Some(DataLicense::CcBy)
);
assert_eq!(
    license.rights_uri(),
    "https://creativecommons.org/licenses/by/4.0/"
);
assert!(license.requires_attribution());
```
