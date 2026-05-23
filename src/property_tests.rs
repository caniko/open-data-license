use super::DataLicense;
use proptest::prelude::*;
use proptest::test_runner::Config as ProptestConfig;

fn data_license() -> impl Strategy<Value = DataLicense> {
    prop_oneof![
        Just(DataLicense::Cc0),
        Just(DataLicense::CcBy),
        Just(DataLicense::CcBySa),
        Just(DataLicense::CcByNc),
        Just(DataLicense::CcByNcSa),
        Just(DataLicense::Pddl),
        Just(DataLicense::OdcBy),
        Just(DataLicense::OdcOdbl),
        Just(DataLicense::CdlaPermissive2_0),
        Just(DataLicense::CdlaSharing1_0),
        Just(DataLicense::Pdm),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        .. ProptestConfig::default()
    })]

    #[test]
    fn compatibility_is_reflexive(license in data_license()) {
        prop_assert!(license.is_compatible_with(&license));
    }

    #[test]
    fn compatibility_is_symmetric(a in data_license(), b in data_license()) {
        prop_assert_eq!(a.is_compatible_with(&b), b.is_compatible_with(&a));
    }

    #[test]
    fn public_domain_absorbs_every_license(license in data_license()) {
        prop_assert!(DataLicense::Cc0.is_compatible_with(&license));
        prop_assert!(DataLicense::Pddl.is_compatible_with(&license));
    }

    #[test]
    fn most_restrictive_agrees_with_compatibility(a in data_license(), b in data_license()) {
        prop_assert_eq!(
            DataLicense::most_restrictive(a, b).is_some(),
            a.is_compatible_with(&b)
        );
    }
}
