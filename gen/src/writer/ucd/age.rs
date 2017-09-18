// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt::Display;
use std::path::Path;

use reader::ucd::derived_age::AGE_DATA;

use writer::utils::tables::ToRangeCharTable;
use writer::utils::write;
use writer::ucd::unicode_version;


pub fn generate(dir: &Path) {
    unicode_version::emit(&dir);
    age_value_emit(&dir);
}


pub fn age_value_emit(dir: &Path) {
    write(
        &dir,
        "age_values.rsv",
        &AGE_DATA.map.to_range_char_table(Display::fmt),
    );
}
