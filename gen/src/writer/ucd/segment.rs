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

use source::ucd::grapheme_break_property::GRAPHEME_CLUSTER_BREAK_DATA;
use source::ucd::readme::UNICODE_VERSION;
use source::ucd::sentence_break_property::SENTENCE_BREAK_DATA;
use source::ucd::word_break_property::WORD_BREAK_DATA;

use writer::common::emit_unicode_version;
use writer::utils::tables::ToRangeCharTable;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_grapheme_cluster_break(dir);
    emit_word_break(dir);
    emit_sentence_break(dir);
}

fn emit_grapheme_cluster_break(dir: &Path) {
    write(
        dir,
        "grapheme_cluster_break.rsv",
        &GRAPHEME_CLUSTER_BREAK_DATA
            .map
            .to_range_char_table(|v, f| write!(f, "GCB::{}", v)),
    );
}

fn emit_word_break(dir: &Path) {
    write(
        dir,
        "word_break.rsv",
        &WORD_BREAK_DATA
            .map
            .to_range_char_table(|v, f| write!(f, "WB::{}", v)),
    );
}

fn emit_sentence_break(dir: &Path) {
    write(
        dir,
        "sentence_break.rsv",
        &SENTENCE_BREAK_DATA
            .map
            .to_range_char_table(|v, f| write!(f, "SB::{}", v)),
    );
}
