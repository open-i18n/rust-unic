// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::path::Path;
use std::collections::BTreeSet;

use source::ucd::derived_core_properties::DERIVED_CORE_PROPERTIES;
use source::ucd::readme::UNICODE_VERSION;
use source::ucd::unicode_data::UNICODE_DATA;

use writer::utils::tables::ToRangeCharSet;
use writer::common::emit_unicode_version;
use writer::utils::write;


pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_alphabetic(dir);
    emit_numeric(dir);
    emit_alphanumeric(dir);
}


fn emit_alphabetic(dir: &Path) {
    write(
        dir,
        "alphabetic.rsv",
        &DERIVED_CORE_PROPERTIES.alphabetic.to_range_char_set(),
    );
}

fn emit_numeric(dir: &Path) {
    write(dir, "numeric.rsv", &get_numeric().to_range_char_set());
}

fn emit_alphanumeric(dir: &Path) {
    write(
        dir,
        "alphanumeric.rsv",
        &get_alphanumeric().to_range_char_set(),
    );
}


fn get_numeric() -> BTreeSet<char> {
    UNICODE_DATA
        .entries
        .iter()
        .filter(|x| {
            ["Nd", "Nl", "No"].contains(&x.general_category.as_str())
        })
        .map(|x| x.character)
        .collect()
}

fn get_alphanumeric() -> BTreeSet<char> {
    get_numeric()
        .union(&DERIVED_CORE_PROPERTIES.alphabetic)
        .map(|ch| ch.clone())
        .collect()
}
