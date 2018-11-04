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
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use source::ucd::jamo::JAMO_DATA;
use source::ucd::readme::UNICODE_VERSION;
use source::ucd::unicode_data::UNICODE_DATA;

use writer::common::emit_unicode_version;
use writer::utils::tables::ToDirectCharTable;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_name_tables(dir);
    emit_jamo_tables(dir);
}

#[derive(Clone, Debug)]
struct NameRecord<'a> {
    pieces: Vec<&'a str>,
}

fn emit_name_tables(dir: &Path) {
    let mut values: BTreeSet<&str> = BTreeSet::default();
    let map: BTreeMap<char, NameRecord> = UNICODE_DATA
        .entries
        .iter()
        .filter(|x| !x.name.starts_with('<'))
        .map(|x| {
            let pieces = x.name.split_whitespace().collect::<Vec<_>>();
            values.extend(pieces.iter());
            (x.character, NameRecord { pieces })
        })
        .collect();

    let mut values_contents = String::new();
    for piece in values.iter() {
        writeln!(
            values_contents,
            "const {}: &str = \"{}\";",
            piece.replace('-', "_"),
            piece
        )
        .unwrap();
    }
    write(dir, "name_values.rsd", &values_contents);

    write(
        dir,
        "name_map.rsv",
        &map.to_direct_char_table(|record, f| {
            write!(
                f,
                "&[{}]",
                record
                    .pieces
                    .iter()
                    .map(|s| s.replace('-', "_"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }),
    );
}

fn emit_jamo_tables(dir: &Path) {
    write(
        dir,
        "jamo.rsv",
        &JAMO_DATA
            .map
            .to_direct_char_table(|record, f| write!(f, "\"{}\"", record)),
    );
}
