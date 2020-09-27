// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

use unic::ucd::bidi::{BidiClass, BidiClassCategory, CharBidiClass, StrBidiClass};

#[rustfmt::skip]
fn main() {
    let text = concat![
        "א",
        "ב",
        "ג",
        "a",
        "b",
        "c",
    ];

    assert!(!text.has_bidi_explicit());
    assert!(text.has_rtl());
    assert!(text.has_ltr());

    assert_eq!(text.chars().nth(0).unwrap().bidi_class(), BidiClass::RightToLeft);
    assert_eq!(
        text.chars().nth(0).unwrap().bidi_class().category(),
        BidiClassCategory::Strong
    );
    assert!(!text.chars().nth(0).unwrap().is_ltr());
    assert!(text.chars().nth(0).unwrap().is_rtl());

    assert_eq!(text.chars().nth(3).unwrap().bidi_class(), BidiClass::LeftToRight);
    assert_eq!(
        text.chars().nth(0).unwrap().bidi_class().category(),
        BidiClassCategory::Strong
    );
    assert!(text.chars().nth(3).unwrap().is_ltr());
    assert!(!text.chars().nth(3).unwrap().is_rtl());
}
