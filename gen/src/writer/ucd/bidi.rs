// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::char;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;
use std::path::Path;

use reader::ucd::unicode_data::UNICODE_DATA;

use writer::utils::tables::{ToRangeCharSet, ToRangeCharTable};
use writer::ucd::unicode_version;
use writer::utils::write;


pub fn generate(dir: &Path) {
    unicode_version::emit(&dir);
    bidi_class_emit(&dir);
    bidi_mirrored_emit(&dir);
}


// Default `Bidi_Class` for unassigned codepoints.
//
// Ref: <http://www.unicode.org/Public/UNIDATA/extracted/DerivedBidiClass.txt>
const BIDI_CLASS_DEFAULTS: &[(u32, u32, &str)] = &[
    (0x0600, 0x07BF, "AL"),
    (0x08A0, 0x08FF, "AL"),
    (0xFB50, 0xFDCF, "AL"),
    (0xFDF0, 0xFDFF, "AL"),
    (0xFE70, 0xFEFF, "AL"),
    (0x1_EE00, 0x1_EEFF, "AL"),
    (0x0590, 0x05FF, "R"),
    (0x07C0, 0x089F, "R"),
    (0xFB1D, 0xFB4F, "R"),
    (0x1_0800, 0x1_0FFF, "R"),
    (0x1_E800, 0x1_EDFF, "R"),
    (0x1_EF00, 0x1_EFFF, "R"),
    (0x20A0, 0x20CF, "ET"),
];


fn bidi_class_emit(dir: &Path) {
    let mut map: BTreeMap<char, &str> = UNICODE_DATA
        .entries
        .iter()
        .map(|x| (x.character, x.bidi_class.as_str()))
        .collect();

    for &(start, end, default_value) in BIDI_CLASS_DEFAULTS {
        for codepoint in start..(end + 1) {
            if let Some(c) = char::from_u32(codepoint) {
                map.entry(c).or_insert(default_value);
            }
        }
    }

    write(
        &dir,
        "bidi_class_values.rsv",
        &map.to_range_char_table(Display::fmt),
    );
}


fn bidi_mirrored_emit(dir: &Path) {
    let set: BTreeSet<char> = UNICODE_DATA
        .entries
        .iter()
        .filter(|x| x.bidi_mirrored)
        .map(|x| x.character)
        .collect();

    write(&dir, "bidi_mirrored.rsv", &set.to_range_char_set());
}
