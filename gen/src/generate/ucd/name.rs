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
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use super::{UnicodeData, UnicodeDataEntry, UnicodeVersion};

use generate::PREAMBLE;
use generate::char_property::ToSingleBSearchMap;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NameRecord {
    pieces: Vec<String>,
}

impl NameRecord {
    pub fn new(name: &str) -> Self {
        let pieces: Vec<String> = name.split_whitespace()
            .map(|x| x.to_owned())
            .collect::<Vec<_>>();
        NameRecord { pieces }
    }
}

impl fmt::Display for NameRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let quoted = self.pieces.iter().map(|p| format!(r#""{}""#, p)).collect::<Vec<_>>();
        write!(f, "&[{}]", quoted.join(", "))
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct NameData(BTreeMap<char, NameRecord>);

impl NameData {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let NameData(ref map) = *self;
        let mut file = File::create(dir.as_ref().join("name_values.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            map.to_single_bsearch_map(fmt::Display::fmt)
        )?;
        Ok(())
    }
}

impl<'a, I> From<I> for NameData
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut map = BTreeMap::<char, NameRecord>::new();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref name, .. } in it {
            map.insert(
                character,
                NameRecord::new(name),
            );
        }

        NameData(map)
    }
}

/// Generate tables for the ucd-name crate
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
