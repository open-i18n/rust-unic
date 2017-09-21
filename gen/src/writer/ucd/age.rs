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

use source::ucd::derived_age::AGE_DATA;
use source::ucd::readme::UNICODE_VERSION;

use writer::utils::tables::ToRangeCharTable;
use writer::utils::write;
use writer::common::emit_unicode_version;


pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_age_values(dir);
}


pub fn emit_age_values(dir: &Path) {
    write(
        dir,
        "age_values.rsv",
        &AGE_DATA.map.to_range_char_table(Display::fmt),
    );
}
