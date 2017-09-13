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
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use super::UnicodeVersion;

use generate::PREAMBLE;
use generate::tables::ToRangeCharTable;

use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct IdnaMappingEntry {
    /// Valid, Ignored, Mapped, Deviation, Disallowed,
    /// DisallowedStd3Valid, or DisallowedStd3Mapped
    status: &'static str,
    /// Only present if status is Mapped, Deviation, or DisallowedStd3Mapped.
    mapping: Option<String>,
    // idna_2008_status unused
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct IdnaMapping(BTreeMap<char, IdnaMappingEntry>);

impl IdnaMapping {
    fn emit<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let mut file = File::create(dir.as_ref().join("idna_mapping.rsv"))?;
        #[cfg_attr(rustfmt, rustfmt_skip)] // rustfmt wants to linebreak the matches! macro
        writeln!(
            file,
            "{}\n{}",
            PREAMBLE,
            self.0.to_range_char_table(|entry, f| {
                write!(f, "{}", entry.status)?;
                if matches!(entry.status, "Mapped" | "Deviation" | "DisallowedStd3Mapped") {
                    // TODO: use str::escape_unicode when stable
                    write!(f, "(\"")?;
                    if let Some(ref s) = entry.mapping {
                        for ch in s.chars() {
                            write!(f, "{}", ch.escape_unicode())?;
                        }
                    }
                    write!(f, "\")")?;
                }
                Ok(())
            })
        )
    }
}

impl FromStr for IdnaMapping {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^
                  ([[:xdigit:]]{4,6})                              # low code point
                  (?:\.\.([[:xdigit:]]{4,6}))?                     # high code point (option)
                  [[:space:]]*;[[:space:]]*
                  ([[:word:]]*)                                    # status
                  [[:space:]]*
                  (?:
                    ;[[:space:]]*
                    ([[:xdigit:]]{4,6}(?:\x20[[:xdigit:]]{4,6})*)? # mapping (option)
                    [[:space:]]*
                    (?:
                      ;[[:space:]]*
                      (NV8|XV8)                                    # IDNA2008 Status (option)
                      [[:space:]]*
                    )?
                  )?
                \#"
            ).unwrap();
        }

        let mut entries = BTreeMap::new();

        for capture in REGEX.captures_iter(str) {
            if let Some(low) = char::from_u32(u32::from_str_radix(&capture[1], 16).unwrap()) {
                let high = capture
                    .get(2)
                    .map(|m| u32::from_str_radix(m.as_str(), 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .unwrap_or(low);
                let entry = IdnaMappingEntry {
                    status: match &capture[3] {
                        "valid" => "Valid",
                        "ignored" => "Ignored",
                        "mapped" => "Mapped",
                        "deviation" => "Deviation",
                        "disallowed" => "Disallowed",
                        "disallowed_STD3_valid" => "DisallowedStd3Valid",
                        "disallowed_STD3_mapped" => "DisallowedStd3Mapped",
                        s => panic!("Invalid Idna Mapping status {:?}", s),
                    },
                    mapping: capture.get(4).map(|m| {
                        m.as_str()
                            .split(' ')
                            .map(|s| u32::from_str_radix(s, 16).unwrap())
                            .map(|u| char::from_u32(u).unwrap())
                            .collect::<String>()
                    }),
                };
                for ch in chars!(low..high) {
                    entries.insert(ch, entry.clone());
                }
                entries.insert(high, entry);
            }
        }

        Ok(IdnaMapping(entries))
    }
}

fn read_idna_data() -> io::Result<IdnaMapping> {
    let mut file = File::open(Path::new("data/idna/IdnaMappingTable.txt"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.parse().expect("Failed to parse IdnaMappingTable"))
}

/// Generate tables for the idna-mapping crate
pub fn generate<P: AsRef<Path>>(dir: P, version: &UnicodeVersion) -> io::Result<()> {
    println!("> unic::ucd::idna::mapping::unicode_version.rsv");
    version.emit(&dir)?;
    println!(">>> Loading idna IdnaMappingTable");
    let idna_data = read_idna_data()?;
    println!("> unic::ucd::idna::mapping::idna_mapping.rsv");
    idna_data.emit(&dir)?;
    Ok(())
}
