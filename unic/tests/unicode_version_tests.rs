// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![cfg(feature = "unstable")]
#![feature(unicode)]


extern crate unic;


#[test]
fn test_unicode_version_against_rust_core() {
    assert!(std::char::UNICODE_VERSION.major <= unic::UNICODE_VERSION.major as u32);
    if std::char::UNICODE_VERSION.major == unic::UNICODE_VERSION.major as u32 {
        assert!(std::char::UNICODE_VERSION.minor <= unic::UNICODE_VERSION.minor as u32);
        if std::char::UNICODE_VERSION.minor == unic::UNICODE_VERSION.minor as u32 {
            assert!(std::char::UNICODE_VERSION.micro <= unic::UNICODE_VERSION.micro as u32);
        }
    }
}

/* TODO: Enable after `std::char::UnicodeVersion` becomes accessible.
#[test]
fn test_unicode_version_against_rust_core_2() {
    fn check(unicode_version: std::char::UnicodeVersion) {
        assert!(unicode_version.major <= unic::UNICODE_VERSION.major as u32);
        if unicode_version.major == unic::UNICODE_VERSION.major as u32 {
            assert!(unicode_version.minor <= unic::UNICODE_VERSION.minor as u32);
            if unicode_version.minor == unic::UNICODE_VERSION.minor as u32 {
                assert!(unicode_version.micro <= unic::UNICODE_VERSION.micro as u32);
            }
        }
    }

    check(std::char::UNICODE_VERSION);
}
*/
