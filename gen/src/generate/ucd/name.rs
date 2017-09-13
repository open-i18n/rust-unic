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
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeDataEntry, UnicodeVersion};

use generate::PREAMBLE;
use generate::tables::ToDirectCharTable;

#[derive(Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct NameRecord<'a> {
    pieces: Vec<&'a str>,
}

struct NameData<'a>(BTreeSet<&'a str>, BTreeMap<char, NameRecord<'a>>);

impl<'a> NameData<'a> {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let (parts, map) = (&self.0, &self.1);

        let mut file = File::create(dir.as_ref().join("name_values.rs"))?;
        writeln!(file, "{}", PREAMBLE)?;
        for part in parts.iter() {
            writeln!(
                file,
                "const {}: &str = \"{}\";",
                part.replace('-', "_"),
                part
            )?;
        }

        let mut file = File::create(dir.as_ref().join("name_map.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            map.to_direct_char_table(|record, f| {
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
        )?;
        Ok(())
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

pub fn generate<P: AsRef<Path>>(
    dir: P,
    version: &UnicodeVersion,
    data: &UnicodeData,
) -> io::Result<()> {
    println!("> unic::ucd::name::tables::unicode_version");
    version.emit(&dir)?;
    println!("> unic::ucd::name::tables::name");
    NameData::from(data.iter()).emit(dir)?;
    Ok(())
}
