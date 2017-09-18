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

use reader::ucd::unicode_data::{UnicodeDataEntry, UNICODE_DATA};
use reader::ucd::readme::UNICODE_VERSION;

use writer::utils::tables::ToDirectCharTable;
use writer::common::emit_unicode_version;
use writer::utils::write;


pub fn generate(dir: &Path) {
    emit_unicode_version(&dir, &UNICODE_VERSION);
    emit_name_tables(&dir);
}


#[derive(Clone, Debug)]
struct NameRecord<'a> {
    pieces: Vec<&'a str>,
}


fn emit_name_tables(dir: &Path) {
    let mut all_pieces: BTreeSet<&str> = BTreeSet::default();
    let mut map: BTreeMap<char, NameRecord> = BTreeMap::default();

    for &UnicodeDataEntry {
        character,
        ref name,
        ..
    } in UNICODE_DATA.entries.iter()
    {
        if name.starts_with('<') {
            continue;
        }
        let pieces = name.split_whitespace().collect::<Vec<_>>();
        all_pieces.extend(pieces.iter());
        map.insert(character, NameRecord { pieces });
    }

    let mut values_content = String::new();
    for piece in all_pieces.iter() {
        writeln!(
            values_content,
            "const {}: &str = \"{}\";",
            piece.replace('-', "_"),
            piece
        ).unwrap();
    }
    write(&dir, "name_values.rsd", &values_content);

    let map_content = map.to_direct_char_table(|record, f| {
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
    });
    write(&dir, "name_map.rsv", &map_content);
}
