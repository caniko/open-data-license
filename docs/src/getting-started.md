# Getting Started

After publication to crates.io:

```toml
[dependencies]
open-data-license = "0.1.0"
```

Before publication, use the tagged Codeberg release:

```toml
[dependencies]
open-data-license = { git = "https://codeberg.org/caniko/open-data-license.git", tag = "v0.1.0" }
```

Basic usage:

```rust
use open_data_license::DataLicense;

let license = DataLicense::CcBy;

assert_eq!(license.spdx_id(), "CC-BY-4.0");
assert_eq!(
    license.rights_uri(),
    "https://creativecommons.org/licenses/by/4.0/"
);
assert!(license.requires_attribution());
```
