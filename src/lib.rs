#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! Open data license metadata with SPDX identifiers and compatibility rules.
//!
//! Supports the major open licenses used in scientific/research data:
//! Creative Commons (CC0/BY/BY-SA/BY-NC/BY-NC-SA), Open Data Commons
//! (PDDL/ODC-BY/ODbL), Community Data License Agreement
//! (CDLA-Permissive/CDLA-Sharing), and Creative Commons Public Domain Mark.
//!
//! # Example
//!
//! ```
//! use open_data_license::DataLicense;
//!
//! let license = DataLicense::CcBySa;
//!
//! assert_eq!(license.spdx_id(), "CC-BY-SA-4.0");
//! assert_eq!(
//!     DataLicense::from_spdx_id("CC-BY-SA-4.0"),
//!     Some(DataLicense::CcBySa)
//! );
//! assert!(license.requires_attribution());
//! assert!(license.requires_share_alike());
//! assert!(license.is_compatible_with(&DataLicense::CcBy));
//! ```
//!
//! The compatibility helpers are metadata-level workflow helpers, not legal
//! advice. Surface the SPDX identifier and rights URI when presenting license
//! choices to end users.
//!
//! # Forward-compatible matching
//!
//! `DataLicense` is marked `#[non_exhaustive]`, so downstream matches should
//! include a wildcard arm:
//!
//! ```
//! use open_data_license::DataLicense;
//!
//! fn can_use_without_attribution(license: DataLicense) -> bool {
//!     match license {
//!         DataLicense::Cc0 | DataLicense::Pddl | DataLicense::Pdm => true,
//!         _ => false,
//!     }
//! }
//!
//! assert!(can_use_without_attribution(DataLicense::Cc0));
//! ```

use std::{error::Error, fmt, str::FromStr};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

mod data_use_restriction;

pub use data_use_restriction::{DataUseRestrictionKind, DataUseRestrictionSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LicenseFamily {
    CreativeCommons,
    OpenDataCommons,
    Cdla,
    PublicDomain,
}

/// Data license types for datasets.
///
/// `Display` and `FromStr` use this crate's stable screaming-snake enum
/// identifiers, such as `"CC_BY_SA"`, so they round-trip with serde's default
/// representation. Use [`DataLicense::spdx_id`] and
/// [`DataLicense::from_spdx_id`] when reading or writing canonical external
/// SPDX-style identifiers such as `"CC-BY-SA-4.0"`.
#[non_exhaustive]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, Display,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum DataLicense {
    /// Creative Commons Zero v1.0 Universal public-domain dedication.
    #[default]
    #[strum(serialize = "CC0")]
    #[serde(rename = "CC0")]
    Cc0,
    /// Creative Commons Attribution 4.0 International.
    #[strum(serialize = "CC_BY")]
    #[serde(rename = "CC_BY")]
    CcBy,
    /// Creative Commons Attribution-ShareAlike 4.0 International.
    #[strum(serialize = "CC_BY_SA")]
    #[serde(rename = "CC_BY_SA")]
    CcBySa,
    /// Creative Commons Attribution-NonCommercial 4.0 International.
    #[strum(serialize = "CC_BY_NC")]
    #[serde(rename = "CC_BY_NC")]
    CcByNc,
    /// Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International.
    #[strum(serialize = "CC_BY_NC_SA")]
    #[serde(rename = "CC_BY_NC_SA")]
    CcByNcSa,

    /// Open Data Commons Public Domain Dedication and License v1.0.
    #[strum(serialize = "PDDL")]
    #[serde(rename = "PDDL")]
    Pddl,
    /// Open Data Commons Attribution License v1.0.
    #[strum(serialize = "ODC_BY")]
    #[serde(rename = "ODC_BY")]
    OdcBy,
    /// Open Data Commons Open Database License v1.0.
    #[strum(serialize = "ODC_ODbL")]
    #[serde(rename = "ODC_ODbL")]
    OdcOdbl,
    /// Community Data License Agreement - Permissive, Version 2.0.
    #[strum(serialize = "CDLA_PERMISSIVE_2_0")]
    #[serde(rename = "CDLA_PERMISSIVE_2_0")]
    CdlaPermissive2_0,
    /// Community Data License Agreement - Sharing, Version 1.0.
    #[strum(serialize = "CDLA_SHARING_1_0")]
    #[serde(rename = "CDLA_SHARING_1_0")]
    CdlaSharing1_0,
    /// Creative Commons Public Domain Mark 1.0 Universal.
    ///
    /// This is a marker for works already in the public domain, not an active
    /// dedication like CC0.
    #[strum(serialize = "PDM")]
    #[serde(rename = "PDM")]
    Pdm,
}

/// Error returned when parsing an unknown SPDX identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownSpdxId(
    /// The unrecognized identifier that was provided by the caller.
    pub String,
);

impl fmt::Display for UnknownSpdxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown SPDX identifier: {}", self.0)
    }
}

impl Error for UnknownSpdxId {}

/// License metadata: (spdx_id, rights_uri, display_name).
struct LicenseMeta {
    spdx: &'static str,
    uri: &'static str,
    name: &'static str,
}

impl DataLicense {
    const fn meta(&self) -> &'static LicenseMeta {
        match self {
            Self::Cc0 => &LicenseMeta {
                spdx: "CC0-1.0",
                uri: "https://creativecommons.org/publicdomain/zero/1.0/",
                name: "Creative Commons Zero v1.0 Universal",
            },
            Self::CcBy => &LicenseMeta {
                spdx: "CC-BY-4.0",
                uri: "https://creativecommons.org/licenses/by/4.0/",
                name: "Creative Commons Attribution 4.0 International",
            },
            Self::CcBySa => &LicenseMeta {
                spdx: "CC-BY-SA-4.0",
                uri: "https://creativecommons.org/licenses/by-sa/4.0/",
                name: "Creative Commons Attribution-ShareAlike 4.0 International",
            },
            Self::CcByNc => &LicenseMeta {
                spdx: "CC-BY-NC-4.0",
                uri: "https://creativecommons.org/licenses/by-nc/4.0/",
                name: "Creative Commons Attribution-NonCommercial 4.0 International",
            },
            Self::CcByNcSa => &LicenseMeta {
                spdx: "CC-BY-NC-SA-4.0",
                uri: "https://creativecommons.org/licenses/by-nc-sa/4.0/",
                name: "Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International",
            },
            Self::Pddl => &LicenseMeta {
                spdx: "PDDL-1.0",
                uri: "https://opendatacommons.org/licenses/pddl/1-0/",
                name: "Open Data Commons Public Domain Dedication and License v1.0",
            },
            Self::OdcBy => &LicenseMeta {
                spdx: "ODC-By-1.0",
                uri: "https://opendatacommons.org/licenses/by/1-0/",
                name: "Open Data Commons Attribution License v1.0",
            },
            Self::OdcOdbl => &LicenseMeta {
                spdx: "ODbL-1.0",
                uri: "https://opendatacommons.org/licenses/odbl/1-0/",
                name: "Open Data Commons Open Database License v1.0",
            },
            Self::CdlaPermissive2_0 => &LicenseMeta {
                spdx: "CDLA-Permissive-2.0",
                uri: "https://cdla.dev/permissive-2-0/",
                name: "Community Data License Agreement - Permissive, Version 2.0",
            },
            Self::CdlaSharing1_0 => &LicenseMeta {
                spdx: "CDLA-Sharing-1.0",
                uri: "https://cdla.dev/sharing-1-0/",
                name: "Community Data License Agreement - Sharing, Version 1.0",
            },
            Self::Pdm => &LicenseMeta {
                spdx: "PDM-1.0",
                uri: "https://creativecommons.org/publicdomain/mark/1.0/",
                name: "Creative Commons Public Domain Mark 1.0 Universal",
            },
        }
    }

    /// SPDX license identifier.
    pub const fn spdx_id(&self) -> &'static str {
        self.meta().spdx
    }

    /// Parse a license from its canonical SPDX-style identifier.
    ///
    /// Matching is case-sensitive. For example, `"CC-BY-SA-4.0"` parses as
    /// [`DataLicense::CcBySa`], while `"cc-by-sa-4.0"` is rejected.
    ///
    /// [`DataLicense::Pdm`] uses `"PDM-1.0"`, a community shorthand for
    /// Creative Commons Public Domain Mark rather than an SPDX-registered
    /// license identifier.
    pub fn from_spdx_id(s: &str) -> Option<DataLicense> {
        match s {
            "CC0-1.0" => Some(Self::Cc0),
            "CC-BY-4.0" => Some(Self::CcBy),
            "CC-BY-SA-4.0" => Some(Self::CcBySa),
            "CC-BY-NC-4.0" => Some(Self::CcByNc),
            "CC-BY-NC-SA-4.0" => Some(Self::CcByNcSa),
            "PDDL-1.0" => Some(Self::Pddl),
            "ODC-By-1.0" => Some(Self::OdcBy),
            "ODbL-1.0" => Some(Self::OdcOdbl),
            "CDLA-Permissive-2.0" => Some(Self::CdlaPermissive2_0),
            "CDLA-Sharing-1.0" => Some(Self::CdlaSharing1_0),
            "PDM-1.0" => Some(Self::Pdm),
            _ => None,
        }
    }

    /// Canonical URL for the license text.
    pub const fn rights_uri(&self) -> &'static str {
        self.meta().uri
    }

    /// Human-readable license name for metadata export.
    pub const fn display_name(&self) -> &'static str {
        self.meta().name
    }

    /// True for public-domain dedications and markers (no restrictions at all).
    pub const fn is_public_domain(&self) -> bool {
        matches!(self, Self::Cc0 | Self::Pddl | Self::Pdm)
    }

    /// True if the license allows commercial use of the data.
    pub const fn allows_commercial_use(&self) -> bool {
        !matches!(self, Self::CcByNc | Self::CcByNcSa)
    }

    /// True if derivatives must use the same license (share-alike / copyleft).
    pub const fn requires_share_alike(&self) -> bool {
        matches!(
            self,
            Self::CcBySa | Self::CcByNcSa | Self::OdcOdbl | Self::CdlaSharing1_0
        )
    }

    /// True if attribution is required.
    pub const fn requires_attribution(&self) -> bool {
        !matches!(
            self,
            Self::Cc0 | Self::Pddl | Self::Pdm | Self::CdlaPermissive2_0
        )
    }

    /// Restrictiveness rank (0 = least restrictive, higher = more restrictive).
    /// Used internally for compatibility comparison.
    const fn restrictiveness(&self) -> u8 {
        match self {
            Self::Cc0 | Self::Pddl | Self::Pdm => 0,
            Self::CcBy | Self::OdcBy | Self::CdlaPermissive2_0 => 1,
            Self::CcBySa | Self::OdcOdbl | Self::CdlaSharing1_0 => 2,
            Self::CcByNc => 3,
            Self::CcByNcSa => 4,
        }
    }

    const fn family(&self) -> LicenseFamily {
        match self {
            Self::Cc0 | Self::Pddl | Self::Pdm => LicenseFamily::PublicDomain,
            Self::CcBy | Self::CcBySa | Self::CcByNc | Self::CcByNcSa => {
                LicenseFamily::CreativeCommons
            }
            Self::OdcBy | Self::OdcOdbl => LicenseFamily::OpenDataCommons,
            Self::CdlaPermissive2_0 | Self::CdlaSharing1_0 => LicenseFamily::Cdla,
        }
    }

    const fn is_same_family_as(&self, other: &Self) -> bool {
        matches!(
            (self.family(), other.family()),
            (
                LicenseFamily::CreativeCommons,
                LicenseFamily::CreativeCommons
            ) | (
                LicenseFamily::OpenDataCommons,
                LicenseFamily::OpenDataCommons
            ) | (LicenseFamily::Cdla, LicenseFamily::Cdla)
                | (LicenseFamily::PublicDomain, LicenseFamily::PublicDomain)
        )
    }

    const fn is_same_variant_as(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Cc0, Self::Cc0)
                | (Self::CcBy, Self::CcBy)
                | (Self::CcBySa, Self::CcBySa)
                | (Self::CcByNc, Self::CcByNc)
                | (Self::CcByNcSa, Self::CcByNcSa)
                | (Self::Pddl, Self::Pddl)
                | (Self::OdcBy, Self::OdcBy)
                | (Self::OdcOdbl, Self::OdcOdbl)
                | (Self::CdlaPermissive2_0, Self::CdlaPermissive2_0)
                | (Self::CdlaSharing1_0, Self::CdlaSharing1_0)
                | (Self::Pdm, Self::Pdm)
        )
    }

    /// Check whether two licenses are compatible for combining datasets.
    ///
    /// Two licenses are compatible when the resulting combined dataset can
    /// legally satisfy both licenses' requirements. Public-domain licenses
    /// are compatible with everything. NC and non-NC licenses are incompatible.
    /// Share-alike obligations are treated as family-local: a share-alike
    /// license is incompatible with licenses from another family, and two
    /// share-alike licenses from the same family are compatible only when they
    /// are the same variant. This conservatively rejects cross-version or
    /// externally declared share-alike compatibility.
    pub const fn is_compatible_with(&self, other: &DataLicense) -> bool {
        // Public domain is always compatible
        if self.is_public_domain() || other.is_public_domain() {
            return true;
        }

        // NC and non-NC are fundamentally incompatible
        if self.allows_commercial_use() != other.allows_commercial_use() {
            return false;
        }

        if (self.requires_share_alike() || other.requires_share_alike())
            && !self.is_same_family_as(other)
        {
            return false;
        }

        if self.requires_share_alike() && other.requires_share_alike() {
            return self.is_same_variant_as(other);
        }

        true
    }

    /// Return the more restrictive of two licenses, for labelling combined datasets.
    ///
    /// Returns `None` if the licenses are incompatible.
    pub const fn most_restrictive(a: Self, b: Self) -> Option<DataLicense> {
        if !a.is_compatible_with(&b) {
            return None;
        }
        if a.restrictiveness() >= b.restrictiveness() {
            Some(a)
        } else {
            Some(b)
        }
    }
}

impl FromStr for DataLicense {
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CC0" => Ok(Self::Cc0),
            "CC_BY" => Ok(Self::CcBy),
            "CC_BY_SA" => Ok(Self::CcBySa),
            "CC_BY_NC" => Ok(Self::CcByNc),
            "CC_BY_NC_SA" => Ok(Self::CcByNcSa),
            "PDDL" => Ok(Self::Pddl),
            "ODC_BY" => Ok(Self::OdcBy),
            "ODC_ODbL" => Ok(Self::OdcOdbl),
            "CDLA_PERMISSIVE_2_0" => Ok(Self::CdlaPermissive2_0),
            "CDLA_SHARING_1_0" => Ok(Self::CdlaSharing1_0),
            "PDM" => Ok(Self::Pdm),
            _ => Err(strum::ParseError::VariantNotFound),
        }
    }
}

impl TryFrom<&str> for DataLicense {
    type Error = UnknownSpdxId;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_spdx_id(value).ok_or_else(|| UnknownSpdxId(value.to_owned()))
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

#[cfg(test)]
#[path = "property_tests.rs"]
mod property_tests;
