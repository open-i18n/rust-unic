// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


extern crate unic_ucd_age;
extern crate unic_ucd_core;
extern crate unic_utils;

use unic_ucd_age::{Age, UnicodeVersion, UNICODE_VERSION};
use unic_utils::iter_all_chars;

/// Character *assignement* values always have Unicode Micro (Update) Version value of zero (`0`).
#[test]
fn test_assigned_age_micro_value() {
    for ch in iter_all_chars() {
        if let Some(uni_ver) = Age::of(ch).assigned() {
            assert_eq!(uni_ver.micro, 0);
        }
    }
}

/// The *earliest* value for this property is `UnicodeVersion { major: 1, minor: 1, micro: 0 }`,
/// because of the massive changes for the merger of the Unicode Stanrda with ISO 10646.
#[test]
fn test_earliest_assigned_age() {
    for ch in iter_all_chars() {
        if let Some(uni_ver) = Age::of(ch).assigned() {
            assert!(
                uni_ver >=
                    UnicodeVersion {
                        major: 1,
                        minor: 1,
                        micro: 0,
                    }
            );
        }
    }
}

/// The *latest* value for this property is always equal to or less than `UNICODE_VERSION`. (Only
/// not equal when `UNICODE_VERSION` has non-zero *micro* value.)
#[test]
fn test_latest_assigned_age() {
    for ch in iter_all_chars() {
        if let Some(uni_ver) = Age::of(ch).assigned() {
            assert!(uni_ver <= UNICODE_VERSION);
        }
    }
}
