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
use std::str::FromStr;

use regex::Regex;

use source::utils::read;

lazy_static! {
    pub static ref IDNA_MAPPING: IdnaMapping =
        { read("data/idna/IdnaMappingTable.txt").parse().unwrap() };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdnaMapping {
    pub map: BTreeMap<char, IdnaMappingEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdnaMappingEntry {
    /// Valid, Ignored, Mapped, Deviation, Disallowed,
    /// DisallowedStd3Valid, or DisallowedStd3Mapped
    pub status: &'static str,

    /// Only present if status is Mapped, Deviation, or DisallowedStd3Mapped.
    pub mapping: Option<String>,
    // idna_2008_status unused
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
            )
            .unwrap();
        }

        let mut map = BTreeMap::new();

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
                    map.insert(ch, entry.clone());
                }
                map.insert(high, entry);
            }
        }

        Ok(IdnaMapping { map })
    }
}
