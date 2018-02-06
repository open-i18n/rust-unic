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
fn basic_tests() {
    assert_eq!(
        Name::of('A').expect("No name for A").to_string(),
        "LATIN CAPITAL LETTER A"
    );
    assert_eq!(
        Name::of('곲').expect("No name for 곲").to_string(),
        "HANGUL SYLLABLE ACF2" // FIXME: use decomposed jamo short names instead
    );
    assert_eq!(
        Name::of('龑').expect("No name for 龑").to_string(),
        "CJK UNIFIED IDEOGRAPH-9F91"
    );
    assert_eq!(
        Name::of('\u{17005}').expect("No name for U+17005").to_string(),
        "TANGUT IDEOGRAPH-17005"
    );
    assert_eq!(
        Name::of('\u{1B175}').expect("No name for U+1B175").to_string(),
        "NUSHU CHARACTER-1B175"
    );
    assert_eq!(
        Name::of('理').expect("No name for 理").to_string(),
        "CJK COMPATIBILITY IDEOGRAPH-F9E4"
    );
    assert_eq!(Name::of('\u{10FFFF}'), None);

    assert!(Name::of('A') < Name::of('B'));
}
