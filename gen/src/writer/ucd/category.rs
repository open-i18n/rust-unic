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
use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;

use source::ucd::unicode_data::UNICODE_DATA;
use source::ucd::readme::UNICODE_VERSION;

use writer::utils::tables::ToRangeCharTable;
use writer::common::emit_unicode_version;
use writer::utils::write;


pub fn generate(dir: &Path) {
    emit_unicode_version(&dir, &UNICODE_VERSION);
    emit_general_category(dir);
}


fn emit_general_category(dir: &Path) {
    let map: BTreeMap<char, &str> = UNICODE_DATA
        .entries
        .iter()
        .map(|x| (x.character, x.general_category.as_str()))
        .collect();

    write(
        &dir,
        "general_category.rsv",
        &map.to_range_char_table(Display::fmt),
    );
}
