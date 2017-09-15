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

use writer::utils::tables::ToDirectCharTable;
use writer::ucd::unicode_version;
use writer::utils::write;


#[derive(Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NameRecord<'a> {
    pieces: Vec<&'a str>,
}

struct NameData<'a>(BTreeSet<&'a str>, BTreeMap<char, NameRecord<'a>>);

impl<'a> NameData<'a> {
    fn emit(&self, dir: &Path) {
        let (parts, map) = (&self.0, &self.1);

        let mut contents = String::new();
        for part in parts.iter() {
            writeln!(
                contents,
                "const {}: &str = \"{}\";",
                part.replace('-', "_"),
                part
            ).unwrap();
        }
        write(&dir, "name_values.rsd", &contents);

        let contents = map.to_direct_char_table(|record, f| {
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
        write(&dir, "name_map.rsv", &contents);
    }
}

impl<'a, I> From<I> for NameData<'a>
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut all_pieces = BTreeSet::default();
        let mut map = BTreeMap::default();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref name, .. } in it {
            if name.starts_with('<') {
                continue;
            }
            let pieces = name.split_whitespace().collect::<Vec<_>>();
            all_pieces.extend(pieces.iter());
            map.insert(character, NameRecord { pieces });
        }

        NameData(all_pieces, map)
    }
}


pub fn generate(dir: &Path) {
    unicode_version::emit(&dir);
    NameData::from(UNICODE_DATA.0.iter()).emit(dir);
}
