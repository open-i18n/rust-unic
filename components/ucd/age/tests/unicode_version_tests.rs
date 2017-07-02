// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


extern crate unic_ucd_core;
extern crate unic_ucd_age;


#[test]
fn test_unicode_version_against_ucd_core() {
    assert_eq!(
        unic_ucd_age::UNICODE_VERSION,
        unic_ucd_core::UNICODE_VERSION
    );
}


#[test]
/// Current `UNICODE_VERSION` value must be a valid `Age` value.
fn test_unicode_version_against_age() {
    use unic_ucd_age::Age;

    let age = Age::from_unicode_version(unic_ucd_age::UNICODE_VERSION);
    assert!(age.is_some());

    let univer = age.unwrap().to_unicode_version();
    assert!(univer.is_some());
    assert_eq!(univer.unwrap(), unic_ucd_age::UNICODE_VERSION);
}
