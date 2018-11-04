// Copyright 2018 The UNIC Project Developers.
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

extern crate unic_char_basics;

use unic_char_basics::unicode_notation;

/// Test Unicode Notation formatting for samples from Unicode Scalar Value range.
#[test]
fn test_unicode_notation() {
    fn format(codepoint: char) -> String {
        unicode_notation(codepoint).to_string()
    }

    // Plane 0 (BMP)
    assert_eq!(format('\u{0}'), "U+0000");
    assert_eq!(format('\u{20}'), "U+0020");
    assert_eq!(format('\u{41}'), "U+0041");
    assert_eq!(format('\u{80}'), "U+0080");
    assert_eq!(format('\u{200c}'), "U+200C");
    assert_eq!(format('\u{d7ff}'), "U+D7FF");

    assert_eq!(format('\u{fffe}'), "U+FFFE");
    assert_eq!(format('\u{ffff}'), "U+FFFF");

    // Plane 1 (SMP)
    assert_eq!(format('\u{1_0000}'), "U+10000");
    assert_eq!(format('\u{1_ffff}'), "U+1FFFF");

    // Plane 14 (SSP)
    assert_eq!(format('\u{e_0000}'), "U+E0000");
    assert_eq!(format('\u{e_ffff}'), "U+EFFFF");

    // Plane 15 (PUA-A)
    assert_eq!(format('\u{f_0000}'), "U+F0000");
    assert_eq!(format('\u{f_ffff}'), "U+FFFFF");

    // Plane 16 (PUA-B)
    assert_eq!(format('\u{10_0000}'), "U+100000");
    assert_eq!(format('\u{10_ffff}'), "U+10FFFF");
}
