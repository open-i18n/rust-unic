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

use reader::ucd::unicode_data::UNICODE_DATA;
use reader::ucd::readme::UNICODE_VERSION;

use writer::utils::tables::ToRangeCharTable;
use writer::common::unicode_version;
use writer::utils::write;


pub fn generate(dir: &Path) {
    unicode_version::emit(&dir, &UNICODE_VERSION);
    category_data_emit(dir);
}


fn category_data_emit(dir: &Path) {
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
