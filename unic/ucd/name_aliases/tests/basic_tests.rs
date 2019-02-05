// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use unic_ucd_name_aliases::{name_aliases_of, NameAliasType};

#[test]
fn test_name_alias_type_of() {
    assert_eq!(
        NameAliasType::of('\u{0000}').unwrap(),
        &[
            NameAliasType::ControlCodeNames,
            NameAliasType::NameAbbreviations
        ]
    );

    assert_eq!(
        NameAliasType::of('\u{0020}').unwrap(),
        &[NameAliasType::NameAbbreviations]
    );

    assert_eq!(
        NameAliasType::of('\u{0080}').unwrap(),
        &[NameAliasType::Figments, NameAliasType::NameAbbreviations]
    );

    assert_eq!(
        NameAliasType::of('\u{FEFF}').unwrap(),
        &[
            NameAliasType::AlternateNames,
            NameAliasType::NameAbbreviations
        ]
    );

    assert_eq!(
        NameAliasType::of('\u{122D4}').unwrap(),
        &[NameAliasType::NameCorrections]
    );

    assert_eq!(NameAliasType::of('\u{0041}'), None);
}

#[test]
fn test_name_corrections() {
    assert_eq!(
        name_aliases_of('\u{0000}', NameAliasType::NameCorrections),
        None
    );

    assert_eq!(
        name_aliases_of('\u{01A2}', NameAliasType::NameCorrections).unwrap(),
        &["LATIN CAPITAL LETTER GHA"]
    );

    assert_eq!(
        name_aliases_of('\u{2B7A}', NameAliasType::NameCorrections).unwrap(),
        &["LEFTWARDS TRIANGLE-HEADED ARROW WITH DOUBLE VERTICAL STROKE"]
    );
}

#[test]
fn test_control_code_names() {
    assert_eq!(
        name_aliases_of('\u{0041}', NameAliasType::ControlCodeNames),
        None
    );

    assert_eq!(
        name_aliases_of('\u{0000}', NameAliasType::ControlCodeNames).unwrap(),
        &["NULL"]
    );

    assert_eq!(
        name_aliases_of('\u{001C}', NameAliasType::ControlCodeNames).unwrap(),
        &["INFORMATION SEPARATOR FOUR", "FILE SEPARATOR"]
    );
}

#[test]
fn test_alternate_names() {
    assert_eq!(
        name_aliases_of('\u{0000}', NameAliasType::AlternateNames),
        None
    );

    assert_eq!(
        name_aliases_of('\u{FEFF}', NameAliasType::AlternateNames).unwrap(),
        &["BYTE ORDER MARK"]
    );
}

#[test]
fn test_figments() {
    assert_eq!(name_aliases_of('\u{0000}', NameAliasType::Figments), None);

    assert_eq!(
        name_aliases_of('\u{0080}', NameAliasType::Figments).unwrap(),
        &["PADDING CHARACTER"]
    );

    assert_eq!(
        name_aliases_of('\u{0099}', NameAliasType::Figments).unwrap(),
        &["SINGLE GRAPHIC CHARACTER INTRODUCER"]
    );
}

#[test]
fn test_name_abbreviations() {
    assert_eq!(
        name_aliases_of('\u{0041}', NameAliasType::NameAbbreviations),
        None
    );

    assert_eq!(
        name_aliases_of('\u{0000}', NameAliasType::NameAbbreviations).unwrap(),
        &["NUL"]
    );

    assert_eq!(
        name_aliases_of('\u{000A}', NameAliasType::NameAbbreviations).unwrap(),
        &["LF", "NL", "EOL"]
    );

    assert_eq!(
        name_aliases_of('\u{FE00}', NameAliasType::NameAbbreviations).unwrap(),
        &["VS1"]
    );
}
