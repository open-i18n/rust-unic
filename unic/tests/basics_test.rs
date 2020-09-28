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

use unic::bidi::BidiInfo;
use unic::normal::StrNormalForm;
use unic::segment::{GraphemeIndices, Graphemes, WordBoundIndices, WordBounds, Words};
use unic::ucd::common::is_alphanumeric;
use unic::ucd::normal::compose;
use unic::ucd::{is_cased, Age, BidiClass, CharAge, CharBidiClass, StrBidiClass, UnicodeVersion};

#[rustfmt::skip]
#[test]
fn test_sample() {

    // Age

    assert_eq!(Age::of('A').unwrap().actual(), UnicodeVersion { major: 1, minor: 1, micro: 0 });
    assert_eq!(Age::of('\u{A0000}'), None);
    assert_eq!(
        Age::of('\u{10FFFF}').unwrap().actual(),
        UnicodeVersion { major: 2, minor: 0, micro: 0 }
    );

    if let Some(age) = '🦊'.age() {
        assert_eq!(age.actual().major, 9);
        assert_eq!(age.actual().minor, 0);
        assert_eq!(age.actual().micro, 0);
    }

    // Bidi

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
    assert!(!text.chars().nth(0).unwrap().is_ltr());
    assert!(text.chars().nth(0).unwrap().is_rtl());

    assert_eq!(text.chars().nth(3).unwrap().bidi_class(), BidiClass::LeftToRight);
    assert!(text.chars().nth(3).unwrap().is_ltr());
    assert!(!text.chars().nth(3).unwrap().is_rtl());

    let bidi_info = BidiInfo::new(text, None);
    assert_eq!(bidi_info.paragraphs.len(), 1);

    let para = &bidi_info.paragraphs[0];
    assert_eq!(para.level.number(), 1);
    assert_eq!(para.level.is_rtl(), true);

    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);
    assert_eq!(
        display,
        concat![
            "a",
            "b",
            "c",
            "ג",
            "ב",
            "א",
        ]
    );

    // Case

    assert_eq!(is_cased('A'), true);
    assert_eq!(is_cased('א'), false);

    // Normalization

    assert_eq!(compose('A', '\u{030A}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");

    // Segmentation

    assert_eq!(
        Graphemes::new("a\u{310}e\u{301}o\u{308}\u{332}").collect::<Vec<&str>>(),
        &["a\u{310}", "e\u{301}", "o\u{308}\u{332}"]
    );

    assert_eq!(
        Graphemes::new("a\r\nb🇺🇳🇮🇨").collect::<Vec<&str>>(),
        &["a", "\r\n", "b", "🇺🇳", "🇮🇨"]
    );

    assert_eq!(
        GraphemeIndices::new("a̐éö̲\r\n").collect::<Vec<(usize, &str)>>(),
        &[(0, "a̐"), (3, "é"), (6, "ö̲"), (11, "\r\n")]
    );

    assert_eq!(
        Words::new(
            "The quick (\"brown\") fox can't jump 32.3 feet, right?",
            |s: &&str| s.chars().any(is_alphanumeric),
        ).collect::<Vec<&str>>(),
        &["The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right"]
    );

    assert_eq!(
        WordBounds::new("The quick (\"brown\")  fox").collect::<Vec<&str>>(),
        &["The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", " ", "fox"]
    );

    assert_eq!(
        WordBoundIndices::new("Brr, it's 29.3°F!").collect::<Vec<(usize, &str)>>(),
        &[
            (0, "Brr"),
            (3, ","),
            (4, " "),
            (5, "it's"),
            (9, " "),
            (10, "29.3"),
            (14, "°"),
            (16, "F"),
            (17, "!")
        ]
    );
}
