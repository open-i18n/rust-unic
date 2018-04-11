// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::path::Path;

use source::ucd::readme::UNICODE_VERSION;
use source::ucd::unihan_numeric_values::UNIHAN_NUMERIC_VALUES_DATA;

use writer::common::emit_unicode_version;
use writer::utils::tables::ToDirectCharTable;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_unihan_numeric_values_tables(dir);
}

fn emit_unihan_numeric_values_tables(dir: &Path) {
    let mut accounting_numeric_map: BTreeMap<char, u64> = BTreeMap::default();
    let mut primary_numeric_map: BTreeMap<char, u64> = BTreeMap::default();
    let mut other_numeric_map: BTreeMap<char, u64> = BTreeMap::default();

    for entry in UNIHAN_NUMERIC_VALUES_DATA.entries.iter() {
        if let Some(value) = entry.accounting_numeric {
            accounting_numeric_map.insert(entry.character, value);
        }
        if let Some(value) = entry.primary_numeric {
            primary_numeric_map.insert(entry.character, value);
        }
        if let Some(value) = entry.other_numeric {
            other_numeric_map.insert(entry.character, value);
        }
    }

    write(
        dir,
        "accounting_numeric_map.rsv",
        &accounting_numeric_map.to_direct_char_table(|record, f| {
            write!(f, "{}", record)
        }),
    );
    write(
        dir,
        "primary_numeric_map.rsv",
        &primary_numeric_map.to_direct_char_table(|record, f| {
            write!(f, "{}", record)
        }),
    );
    write(
        dir,
        "other_numeric_map.rsv",
        &other_numeric_map.to_direct_char_table(|record, f| {
            write!(f, "{}", record)
        }),
    );
}
