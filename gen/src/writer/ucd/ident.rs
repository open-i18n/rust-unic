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

use crate::source::ucd::derived_core_properties::DERIVED_CORE_PROPERTIES;
use crate::source::ucd::prop_list::PROP_LIST;
use crate::source::ucd::readme::UNICODE_VERSION;

use crate::writer::common::emit_unicode_version;
use crate::writer::utils::tables::ToRangeCharSet;
use crate::writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_id_start(dir);
    emit_id_continue(dir);
    emit_xid_start(dir);
    emit_xid_continue(dir);
    emit_pattern_syntax(dir);
    emit_pattern_white_space(dir);
}

fn emit_id_start(dir: &Path) {
    write(
        dir,
        "id_start.rsv",
        &DERIVED_CORE_PROPERTIES.id_start.to_range_char_set(),
    );
}

fn emit_id_continue(dir: &Path) {
    write(
        dir,
        "id_continue.rsv",
        &DERIVED_CORE_PROPERTIES.id_continue.to_range_char_set(),
    );
}

fn emit_xid_start(dir: &Path) {
    write(
        dir,
        "xid_start.rsv",
        &DERIVED_CORE_PROPERTIES.xid_start.to_range_char_set(),
    );
}

fn emit_xid_continue(dir: &Path) {
    write(
        dir,
        "xid_continue.rsv",
        &DERIVED_CORE_PROPERTIES.xid_continue.to_range_char_set(),
    );
}

fn emit_pattern_syntax(dir: &Path) {
    write(
        dir,
        "pattern_syntax.rsv",
        &PROP_LIST.pattern_syntax.to_range_char_set(),
    );
}

fn emit_pattern_white_space(dir: &Path) {
    write(
        dir,
        "pattern_white_space.rsv",
        &PROP_LIST.pattern_white_space.to_range_char_set(),
    );
}
