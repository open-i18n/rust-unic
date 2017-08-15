extern crate unic_ucd_core;
extern crate unic_ucd_name;

#[test]
fn test_unicode_version_against_ucd_core() {
    assert_eq!(
        unic_ucd_name::UNICODE_VERSION,
        unic_ucd_core::UNICODE_VERSION
    )
}
