// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_ucd_name;

use unic_ucd_name::Name;

#[test]
fn test_name_str() {
    assert_eq!(
        Name::of('A').unwrap().to_string(),
        "LATIN CAPITAL LETTER A"
    );
    assert_eq!(
        Name::of('곲').unwrap().to_string(),
        "HANGUL SYLLABLE ACF2" // FIXME: use decomposed jamo short names instead
    );
    assert_eq!(
        Name::of('言').unwrap().to_string(),
        "CJK UNIFIED IDEOGRAPH-8A00"
    );
    assert_eq!(
        Name::of('\u{17005}').unwrap().to_string(),
        "TANGUT IDEOGRAPH-17005"
    );
    assert_eq!(
        Name::of('\u{1B175}').unwrap().to_string(),
        "NUSHU CHARACTER-1B175"
    );
    assert_eq!(
        Name::of('理').unwrap().to_string(),
        "CJK COMPATIBILITY IDEOGRAPH-F9E4"
    );
    assert_eq!(Name::of('\u{10FFFF}'), None);
}

#[test]
fn test_name_cmp() {
    assert!(Name::of('A') < Name::of('B'));
    assert!(Name::of('言') < Name::of('A'));
    assert!(Name::of('理') < Name::of('言'));
}
