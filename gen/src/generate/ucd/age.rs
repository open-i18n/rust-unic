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
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use super::UnicodeVersion;

use generate::PREAMBLE;
use generate::tables::ToRangeCharTable;

use regex::Regex;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct AgeData(BTreeMap<char, String>);

impl AgeData {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let AgeData(ref map) = *self;
        let mut file = File::create(dir.as_ref().join("age_values.rsv"))?;
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            map.to_range_char_table(Display::fmt)
        )?;
        Ok(())
    }
}

impl FromStr for AgeData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^                     # every line
                  ([[:xdigit:]]{4,6})        # range start
                  (?:..([[:xdigit:]]{4,6}))? # range end (option)
                  [[:blank:]]*;[[:blank:]]*  # separator
                  ([[:digit:]]+)             # major version
                  \.([[:digit:]]+)           # minor version
                  (?:\.([[:digit:]]+))?      # micro version (option)
                ",
            ).unwrap();
        }

        let mut age_data = BTreeMap::default();
        for capture in REGEX.captures_iter(str) {
            let start = u32::from_str_radix(&capture[1], 16).unwrap();
            let end = capture
                .get(2)
                .map_or(start, |m| u32::from_str_radix(m.as_str(), 16).unwrap());
            let major = capture[3].parse::<u16>().unwrap();
            let minor = capture[4].parse::<u16>().unwrap();
            let micro = capture
                .get(5)
                .map_or(0, |m| m.as_str().parse::<u16>().unwrap());

            for point in start..(end + 1) {
                if let Some(char) = char::from_u32(point) {
                    age_data.insert(
                        char,
                        format!(
                            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
                            major,
                            minor,
                            micro,
                        ),
                    );
                }
            }
        }

        Ok(AgeData(age_data))
    }
}

/// Generate tables for the ucd-age crate
pub fn generate<P: AsRef<Path>>(dir: P, version: &UnicodeVersion) -> io::Result<()> {
    println!("> unic::ucd::age::tables::unicode_version");
    version.emit(&dir)?;
    println!(">>> Loading UCD DerivedAge");
    let mut derived_age = File::open(Path::new("data/ucd/DerivedAge.txt"))?;
    let mut buffer = String::new();
    derived_age.read_to_string(&mut buffer)?;
    println!("> unic::ucd::age::tables::age_values");
    buffer.parse::<AgeData>().unwrap().emit(dir)?;
    Ok(())
}
