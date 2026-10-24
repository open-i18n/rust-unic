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

use crate::source::utils::read;

lazy_static! {
    pub static ref DERIVED_NAME_DATA: DerivedNameData = {
        read("external/unicode/ucd/data/extracted/DerivedName.txt")
            .parse()
            .unwrap()
    };
}

/// The ranges of characters whose `Name` is derived by rule NR2 (a script-specific prefix
/// followed by the code point in hexadecimal), taken from the `XXXX..YYYY ; PREFIX-*` lines of
/// `DerivedName.txt`. Maps every such character to its prefix (including the trailing `-`).
#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DerivedNameData {
    pub prefix_map: BTreeMap<char, String>,
}

impl FromStr for DerivedNameData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^
                  ([[:xdigit:]]{4,6})        # range start
                  \.\.([[:xdigit:]]{4,6})    # range end
                  [[:blank:]]*;[[:blank:]]*  # separator
                  ([[:upper:][:digit:][:blank:]]+-)\*  # name prefix
                ",
            )
            .expect("Bad regex");
        }

        let mut prefix_map = BTreeMap::default();
        for capture in REGEX.captures_iter(str) {
            let start = u32::from_str_radix(&capture[1], 16).unwrap();
            let end = u32::from_str_radix(&capture[2], 16).unwrap();
            for point in start..=end {
                if let Some(chr) = char::from_u32(point) {
                    prefix_map.insert(chr, capture[3].to_owned());
                }
            }
        }
        Ok(DerivedNameData { prefix_map })
    }
}
