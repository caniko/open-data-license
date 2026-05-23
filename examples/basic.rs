use open_data_license::DataLicense;
use strum::IntoEnumIterator;

fn main() {
    for license in DataLicense::iter() {
        println!(
            "{} | {} | {}",
            license.spdx_id(),
            license.rights_uri(),
            license.display_name()
        );
    }

    let parsed =
        DataLicense::from_spdx_id("CC-BY-4.0").expect("CC-BY-4.0 is a supported SPDX identifier");
    println!("parsed CC-BY-4.0 as {}", parsed);

    let compatible = DataLicense::CcBySa.is_compatible_with(&DataLicense::CcBy);
    println!("CC-BY-SA-4.0 compatible with CC-BY-4.0: {compatible}");
}
