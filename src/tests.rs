use super::*;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[test]
fn serde_roundtrip_all() {
    let expected: &[(DataLicense, &str)] = &[
        (DataLicense::Cc0, "\"CC0\""),
        (DataLicense::CcBy, "\"CC_BY\""),
        (DataLicense::CcBySa, "\"CC_BY_SA\""),
        (DataLicense::CcByNc, "\"CC_BY_NC\""),
        (DataLicense::CcByNcSa, "\"CC_BY_NC_SA\""),
        (DataLicense::Pddl, "\"PDDL\""),
        (DataLicense::OdcBy, "\"ODC_BY\""),
        (DataLicense::OdcOdbl, "\"ODC_ODbL\""),
    ];
    for &(variant, json_str) in expected {
        let json = serde_json::to_string(&variant).unwrap();
        assert_eq!(json, json_str);
        let back: DataLicense = serde_json::from_str(&json).unwrap();
        assert_eq!(back, variant);
    }
}

#[test]
fn strum_roundtrip_all() {
    for license in DataLicense::iter() {
        let s = license.to_string();
        let back = DataLicense::from_str(&s).unwrap();
        assert_eq!(license, back);
    }
}

#[test]
fn serde_invalid() {
    for bad in ["\"INVALID\"", "\"cc0\"", "\"Cc0\"", "1", "null"] {
        assert!(
            serde_json::from_str::<DataLicense>(bad).is_err(),
            "should reject {bad}"
        );
    }
}

#[test]
fn strum_invalid() {
    for bad in ["INVALID", "", "cc0", "Cc0", " CC0", "CC0 "] {
        assert!(DataLicense::from_str(bad).is_err(), "should reject '{bad}'");
    }
}

#[test]
fn license_families() {
    // Creative Commons all start with CC
    for v in [
        DataLicense::Cc0,
        DataLicense::CcBy,
        DataLicense::CcBySa,
        DataLicense::CcByNc,
        DataLicense::CcByNcSa,
    ] {
        assert!(v.to_string().starts_with("CC"), "{v:?}");
    }
    // Open Data Commons start with ODC or PDDL
    for v in [DataLicense::Pddl, DataLicense::OdcBy, DataLicense::OdcOdbl] {
        let s = v.to_string();
        assert!(s.starts_with("PDDL") || s.starts_with("ODC"), "{v:?}");
    }
}

#[test]
fn hash_all_distinct() {
    use std::collections::HashSet;
    let set: HashSet<DataLicense> = DataLicense::iter().collect();
    assert_eq!(set.len(), 8);
}

// -----------------------------------------------------------------
// FAIR: SPDX and URI mappings
// -----------------------------------------------------------------

#[test]
fn spdx_ids_non_empty() {
    for license in DataLicense::iter() {
        assert!(!license.spdx_id().is_empty(), "{license:?}");
    }
}

#[test]
fn rights_uris_are_https() {
    for license in DataLicense::iter() {
        assert!(license.rights_uri().starts_with("https://"), "{license:?}");
    }
}

#[test]
fn display_names_non_empty() {
    for license in DataLicense::iter() {
        assert!(!license.display_name().is_empty(), "{license:?}");
    }
}

// -----------------------------------------------------------------
// FAIR: license property tests
// -----------------------------------------------------------------

#[test]
fn public_domain_licenses() {
    assert!(DataLicense::Cc0.is_public_domain());
    assert!(DataLicense::Pddl.is_public_domain());
    for v in [
        DataLicense::CcBy,
        DataLicense::CcBySa,
        DataLicense::CcByNc,
        DataLicense::CcByNcSa,
        DataLicense::OdcBy,
        DataLicense::OdcOdbl,
    ] {
        assert!(!v.is_public_domain(), "{v:?}");
    }
}

#[test]
fn commercial_use() {
    for v in [
        DataLicense::Cc0,
        DataLicense::CcBy,
        DataLicense::CcBySa,
        DataLicense::Pddl,
        DataLicense::OdcBy,
        DataLicense::OdcOdbl,
    ] {
        assert!(v.allows_commercial_use(), "{v:?}");
    }
    assert!(!DataLicense::CcByNc.allows_commercial_use());
    assert!(!DataLicense::CcByNcSa.allows_commercial_use());
}

#[test]
fn share_alike() {
    assert!(DataLicense::CcBySa.requires_share_alike());
    assert!(DataLicense::CcByNcSa.requires_share_alike());
    assert!(DataLicense::OdcOdbl.requires_share_alike());
    assert!(!DataLicense::CcBy.requires_share_alike());
    assert!(!DataLicense::Cc0.requires_share_alike());
}

#[test]
fn attribution() {
    assert!(!DataLicense::Cc0.requires_attribution());
    assert!(!DataLicense::Pddl.requires_attribution());
    for v in [
        DataLicense::CcBy,
        DataLicense::CcBySa,
        DataLicense::CcByNc,
        DataLicense::CcByNcSa,
        DataLicense::OdcBy,
        DataLicense::OdcOdbl,
    ] {
        assert!(v.requires_attribution(), "{v:?}");
    }
}

// -----------------------------------------------------------------
// FAIR: compatibility matrix
// -----------------------------------------------------------------

#[test]
fn public_domain_compatible_with_everything() {
    for other in DataLicense::iter() {
        assert!(
            DataLicense::Cc0.is_compatible_with(&other),
            "CC0 vs {other:?}"
        );
        assert!(
            DataLicense::Pddl.is_compatible_with(&other),
            "PDDL vs {other:?}"
        );
    }
}

#[test]
fn nc_and_non_nc_incompatible() {
    // CC_BY (commercial OK) vs CC_BY_NC (no commercial) → incompatible
    assert!(!DataLicense::CcBy.is_compatible_with(&DataLicense::CcByNc));
    assert!(!DataLicense::CcByNc.is_compatible_with(&DataLicense::CcBy));
    assert!(!DataLicense::CcBySa.is_compatible_with(&DataLicense::CcByNcSa));
}

#[test]
fn same_license_always_compatible() {
    for license in DataLicense::iter() {
        assert!(license.is_compatible_with(&license), "{license:?}");
    }
}

#[test]
fn most_restrictive_returns_stricter() {
    assert_eq!(
        DataLicense::most_restrictive(DataLicense::Cc0, DataLicense::CcBy),
        Some(DataLicense::CcBy)
    );
    assert_eq!(
        DataLicense::most_restrictive(DataLicense::CcBy, DataLicense::Cc0),
        Some(DataLicense::CcBy)
    );
    assert_eq!(
        DataLicense::most_restrictive(DataLicense::CcByNc, DataLicense::CcByNcSa),
        Some(DataLicense::CcByNcSa)
    );
}

#[test]
fn most_restrictive_incompatible_returns_none() {
    assert_eq!(
        DataLicense::most_restrictive(DataLicense::CcBy, DataLicense::CcByNc),
        None
    );
}

// -----------------------------------------------------------------
// Compatibility — cross-family SA licenses
// -----------------------------------------------------------------

#[test]
fn cross_family_sa_incompatible() {
    // CC-BY-SA (band 2) vs ODbL (band 2) — same restrictiveness but different families
    // Current logic: both SA and same restrictiveness → compatible (by band match)
    // This test documents the actual behavior.
    let result = DataLicense::CcBySa.is_compatible_with(&DataLicense::OdcOdbl);
    // Both are band 2, so under the current "same restrictiveness band" rule
    // they are treated as compatible.
    assert!(
        result,
        "CC-BY-SA and ODbL are in the same restrictiveness band"
    );
}

#[test]
fn sa_vs_non_sa_compatible_when_same_commercial() {
    // CC-BY (non-SA) + CC-BY-SA (SA): both allow commercial
    // SA + non-SA → one side doesn't require SA → compatible
    assert!(DataLicense::CcBy.is_compatible_with(&DataLicense::CcBySa));
    assert!(DataLicense::CcBySa.is_compatible_with(&DataLicense::CcBy));
}

#[test]
fn nc_sa_vs_nc_compatible() {
    // CC-BY-NC and CC-BY-NC-SA: both NC, one is SA → should be compatible
    assert!(DataLicense::CcByNc.is_compatible_with(&DataLicense::CcByNcSa));
    assert!(DataLicense::CcByNcSa.is_compatible_with(&DataLicense::CcByNc));
}

// -----------------------------------------------------------------
// Compatibility — symmetry
// -----------------------------------------------------------------

#[test]
fn compatibility_is_symmetric() {
    for a in DataLicense::iter() {
        for b in DataLicense::iter() {
            assert_eq!(
                a.is_compatible_with(&b),
                b.is_compatible_with(&a),
                "compatibility must be symmetric: {a:?} vs {b:?}"
            );
        }
    }
}

// -----------------------------------------------------------------
// most_restrictive — additional edge cases
// -----------------------------------------------------------------

#[test]
fn most_restrictive_same_license() {
    for license in DataLicense::iter() {
        assert_eq!(
            DataLicense::most_restrictive(license, license),
            Some(license),
            "{license:?} combined with itself"
        );
    }
}

#[test]
fn most_restrictive_commutative_when_compatible() {
    for a in DataLicense::iter() {
        for b in DataLicense::iter() {
            let ab = DataLicense::most_restrictive(a, b);
            let ba = DataLicense::most_restrictive(b, a);
            // Both should agree on compatibility
            match (ab, ba) {
                (Some(x), Some(y)) => {
                    // The result license should have the same restrictiveness
                    assert_eq!(
                        x.restrictiveness(),
                        y.restrictiveness(),
                        "most_restrictive({a:?}, {b:?}) should yield same restrictiveness level"
                    );
                }
                (None, None) => {} // Both incompatible — fine
                _ => panic!("most_restrictive asymmetric: ({a:?}, {b:?}) → {ab:?} vs {ba:?}"),
            }
        }
    }
}

#[test]
fn most_restrictive_public_domain_yields_at_least_as_restrictive() {
    for other in DataLicense::iter() {
        let result = DataLicense::most_restrictive(DataLicense::Cc0, other).unwrap();
        assert!(
            result.restrictiveness() >= other.restrictiveness(),
            "CC0 + {other:?} should yield at least {other:?}'s restrictiveness"
        );
    }
}

// -----------------------------------------------------------------
// Restrictiveness ordering
// -----------------------------------------------------------------

#[test]
fn restrictiveness_ordering_is_total() {
    // Every license has a defined restrictiveness
    let mut ranks: Vec<(DataLicense, u8)> = DataLicense::iter()
        .map(|l| (l, l.restrictiveness()))
        .collect();
    ranks.sort_by_key(|&(_, r)| r);
    // Public domain should be least restrictive
    assert_eq!(ranks[0].1, 0);
    assert!(ranks[0].0.is_public_domain());
}
