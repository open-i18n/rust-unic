// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate unic_char_range;

extern crate unic_ucd_age;

use unic_ucd_age::{Age, UnicodeVersion, UNICODE_VERSION};

/// Character *assignement* values always have Unicode Micro (Update) Version value of zero (`0`).
#[test]
fn test_assigned_age_micro_value() {
    for ch in chars!(..) {
        if let Some(age) = Age::of(ch) {
            assert_eq!(age.actual().micro, 0);
        }
    }
}

/// The *earliest* (largest) value for this property is `UnicodeVersion { major: 1, minor: 1, micro:
/// 0 }`, because of the massive changes for the merger of the Unicode Standard with ISO 10646.
#[test]
fn test_earliest_assigned_age() {
    for ch in chars!(..) {
        if let Some(age) = Age::of(ch) {
            assert!(
                age.actual()
                    >= UnicodeVersion {
                        major: 1,
                        minor: 1,
                        micro: 0,
                    }
            );
        }
    }
}

/// The *latest* (smallest) value for this property is always equal to or greater than
/// `UNICODE_VERSION`. (Only not equal when `UNICODE_VERSION` has non-zero *micro* value.)
#[test]
fn test_latest_assigned_age() {
    for ch in chars!(..) {
        if let Some(age) = Age::of(ch) {
            assert!(age.actual() <= UNICODE_VERSION);
        }
    }
}
