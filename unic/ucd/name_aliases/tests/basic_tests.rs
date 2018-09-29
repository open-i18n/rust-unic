// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_ucd_name_aliases;

use unic_ucd_name_aliases::{
    name_corrections_of,
    control_code_names_of,
    alternate_names_of,
    figments_of,
    name_abbreviations_of,
};

#[test]
fn test_name_corrections() {
    assert_eq!(name_corrections_of('\u{0000}'), None);

    assert_eq!(
        name_corrections_of('\u{01A2}').unwrap(),
        &["LATIN CAPITAL LETTER GHA"]
    );

    assert_eq!(
        name_corrections_of('\u{2B7A}').unwrap(),
        &["LEFTWARDS TRIANGLE-HEADED ARROW WITH DOUBLE VERTICAL STROKE"]
    );
}

#[test]
fn test_control_code_names() {
    assert_eq!(control_code_names_of('\u{0041}'), None);

    assert_eq!(
        control_code_names_of('\u{0000}').unwrap(),
        &["NULL"]
    );

    assert_eq!(
        control_code_names_of('\u{001C}').unwrap(),
        &["INFORMATION SEPARATOR FOUR", "FILE SEPARATOR"]
    );
}

#[test]
fn test_alternate_names() {
    assert_eq!(alternate_names_of('\u{0000}'), None);

    assert_eq!(
        alternate_names_of('\u{FEFF}').unwrap(),
        &["BYTE ORDER MARK"]
    );
}

#[test]
fn test_figments() {
    assert_eq!(figments_of('\u{0000}'), None);

    assert_eq!(
        figments_of('\u{0080}').unwrap(),
        &["PADDING CHARACTER"]
    );

    assert_eq!(
        figments_of('\u{0099}').unwrap(),
        &["SINGLE GRAPHIC CHARACTER INTRODUCER"]
    );
}

#[test]
fn test_name_abbreviations() {
    assert_eq!(name_abbreviations_of('\u{0041}'), None);

    assert_eq!(
        name_abbreviations_of('\u{0000}').unwrap(),
        &["NUL"]
    );

    assert_eq!(
        name_abbreviations_of('\u{000A}').unwrap(),
        &["LF", "NL", "EOL"]
    );

    assert_eq!(
        name_abbreviations_of('\u{FE00}').unwrap(),
        &["VS1"]
    );
}
