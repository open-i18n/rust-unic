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

use source::ucd::derived_core_properties::DERIVED_CORE_PROPERTIES;
use source::ucd::readme::UNICODE_VERSION;

use writer::utils::tables::ToRangeCharSet;
use writer::common::emit_unicode_version;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_lowercase(dir);
    emit_uppercase(dir);
    emit_cased(dir);
    emit_case_ignorable(dir);
    emit_changes_when_lowercased(dir);
    emit_changes_when_uppercased(dir);
    emit_changes_when_titlecased(dir);
    emit_changes_when_casefolded(dir);
    emit_changes_when_casemapped(dir);
}

fn emit_lowercase(dir: &Path) {
    write(
        dir,
        "lowercase.rsv",
        &DERIVED_CORE_PROPERTIES.lowercase.to_range_char_set(),
    );
}

fn emit_uppercase(dir: &Path) {
    write(
        dir,
        "uppercase.rsv",
        &DERIVED_CORE_PROPERTIES.uppercase.to_range_char_set(),
    );
}

fn emit_cased(dir: &Path) {
    write(
        dir,
        "cased.rsv",
        &DERIVED_CORE_PROPERTIES.cased.to_range_char_set(),
    );
}

fn emit_case_ignorable(dir: &Path) {
    write(
        dir,
        "case_ignorable.rsv",
        &DERIVED_CORE_PROPERTIES.case_ignorable.to_range_char_set(),
    );
}

fn emit_changes_when_lowercased(dir: &Path) {
    write(
        dir,
        "changes_when_lowercased.rsv",
        &DERIVED_CORE_PROPERTIES
            .changes_when_lowercased
            .to_range_char_set(),
    );
}

fn emit_changes_when_uppercased(dir: &Path) {
    write(
        dir,
        "changes_when_uppercased.rsv",
        &DERIVED_CORE_PROPERTIES
            .changes_when_uppercased
            .to_range_char_set(),
    );
}

fn emit_changes_when_titlecased(dir: &Path) {
    write(
        dir,
        "changes_when_titlecased.rsv",
        &DERIVED_CORE_PROPERTIES
            .changes_when_titlecased
            .to_range_char_set(),
    );
}

fn emit_changes_when_casefolded(dir: &Path) {
    write(
        dir,
        "changes_when_casefolded.rsv",
        &DERIVED_CORE_PROPERTIES
            .changes_when_casefolded
            .to_range_char_set(),
    );
}

fn emit_changes_when_casemapped(dir: &Path) {
    write(
        dir,
        "changes_when_casemapped.rsv",
        &DERIVED_CORE_PROPERTIES
            .changes_when_casemapped
            .to_range_char_set(),
    );
}
