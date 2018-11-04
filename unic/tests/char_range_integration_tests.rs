// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    unconditional_recursion,
    unsafe_code
)]

extern crate unic_char;
#[macro_use]
extern crate unic_char_range;
extern crate unic_ucd;

use unic_char::basics::{is_noncharacter, is_private_use, unicode_notation};
use unic_char::range::CharRange;
use unic_ucd::age::Age;

/// This test is only based on current practices of The Unicode Standard.
///
/// The main task for this test is to ensure `CharRange::assigned_normal_planes` covers all assigned
/// characters, except the special-purpose ones.
///
/// When the test breaks, either `CharRange::assigned_normal_planes` is out-of-date, or some
/// assumption made here is not held anymore.
#[test]
fn test_char_range_assigned_normal_planes() {
    let assigned_normal_planes = CharRange::assigned_normal_planes();

    for codepoint in CharRange::all() {
        let notation = unicode_notation(codepoint).to_string();
        // Any character is either...
        assert_eq!(
            (
                &notation,
                (
                    // assigned in a normal plane
                    assigned_normal_planes.contains(codepoint)

                    // or, is a Noncharacter
                    || is_noncharacter(codepoint)

                    // or, is a Private-Use characters
                    || is_private_use(codepoint)

                    // or, is Unassigned
                    || Age::of(codepoint) == None

                    // or, is a Plane 14 special character
                    || chars!('\u{e0000}'..='\u{e01ef}').contains(codepoint)
                )
            ),
            (&notation, true)
        );
    }
}
