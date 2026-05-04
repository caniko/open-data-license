//! Open data license enum with SPDX identifiers and compatibility rules.
//!
//! Supports the major open licenses used in scientific/research data:
//! Creative Commons (CC0/BY/BY-SA/BY-NC/BY-NC-SA) and Open Data Commons
//! (PDDL/ODC-BY/ODbL).

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};
use utoipa::ToSchema;

/// Data license types for datasets.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    Display,
    ToSchema,
)]
pub enum DataLicense {
    // Creative Commons Licenses
    #[strum(serialize = "CC0")]
    #[serde(rename = "CC0")]
    Cc0,
    #[strum(serialize = "CC_BY")]
    #[serde(rename = "CC_BY")]
    CcBy,
    #[strum(serialize = "CC_BY_SA")]
    #[serde(rename = "CC_BY_SA")]
    CcBySa,
    #[strum(serialize = "CC_BY_NC")]
    #[serde(rename = "CC_BY_NC")]
    CcByNc,
    #[strum(serialize = "CC_BY_NC_SA")]
    #[serde(rename = "CC_BY_NC_SA")]
    CcByNcSa,

    // Open Data Commons Licenses
    #[strum(serialize = "PDDL")]
    #[serde(rename = "PDDL")]
    Pddl,
    #[strum(serialize = "ODC_BY")]
    #[serde(rename = "ODC_BY")]
    OdcBy,
    #[strum(serialize = "ODC_ODbL")]
    #[serde(rename = "ODC_ODbL")]
    OdcOdbl,
}

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
        }
    }

    /// SPDX license identifier.
    pub const fn spdx_id(&self) -> &'static str {
        self.meta().spdx
    }

    /// Canonical URL for the license text.
    pub const fn rights_uri(&self) -> &'static str {
        self.meta().uri
    }

    /// Human-readable license name for metadata export.
    pub const fn display_name(&self) -> &'static str {
        self.meta().name
    }

    /// True for public-domain dedications (no restrictions at all).
    pub const fn is_public_domain(&self) -> bool {
        matches!(self, Self::Cc0 | Self::Pddl)
    }

    /// True if the license allows commercial use of the data.
    pub const fn allows_commercial_use(&self) -> bool {
        !matches!(self, Self::CcByNc | Self::CcByNcSa)
    }

    /// True if derivatives must use the same license (share-alike / copyleft).
    pub const fn requires_share_alike(&self) -> bool {
        matches!(self, Self::CcBySa | Self::CcByNcSa | Self::OdcOdbl)
    }

    /// True if attribution is required.
    pub const fn requires_attribution(&self) -> bool {
        !matches!(self, Self::Cc0 | Self::Pddl)
    }

    /// Restrictiveness rank (0 = least restrictive, higher = more restrictive).
    /// Used internally for compatibility comparison.
    const fn restrictiveness(&self) -> u8 {
        match self {
            Self::Cc0 | Self::Pddl => 0,
            Self::CcBy | Self::OdcBy => 1,
            Self::CcBySa | Self::OdcOdbl => 2,
            Self::CcByNc => 3,
            Self::CcByNcSa => 4,
        }
    }

    /// Check whether two licenses are compatible for combining datasets.
    ///
    /// Two licenses are compatible when the resulting combined dataset can
    /// legally satisfy both licenses' requirements. Public-domain licenses
    /// are compatible with everything. ShareAlike licenses are only compatible
    /// with themselves or less-restrictive licenses. NC and non-NC licenses
    /// are incompatible.
    pub const fn is_compatible_with(&self, other: &DataLicense) -> bool {
        // Public domain is always compatible
        if self.is_public_domain() || other.is_public_domain() {
            return true;
        }

        // NC and non-NC are fundamentally incompatible
        if self.allows_commercial_use() != other.allows_commercial_use() {
            return false;
        }

        // ShareAlike requires same license family — SA can combine with
        // same-or-less restrictive in the same family, but cross-family
        // SA (CC-BY-SA + ODbL) is not compatible.
        if self.requires_share_alike() && other.requires_share_alike() {
            // Both SA: compatible only within same restrictiveness band
            return self.restrictiveness() == other.restrictiveness();
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

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
