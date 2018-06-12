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

use source::ucd::blocks::BLOCKS_DATA;
use source::ucd::readme::UNICODE_VERSION;

use writer::common::emit_unicode_version;
use writer::utils::tables::ToRangeCharTable;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_blocks(dir);
}

fn emit_blocks(dir: &Path) {
    write(
        dir,
        "blocks.rsv",
        &BLOCKS_DATA
            .map
            .to_range_char_table(|v, f| write!(f, "\"{}\"", v)),
    );
}
