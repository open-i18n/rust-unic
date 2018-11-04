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
    pub static ref GRAPHEME_CLUSTER_BREAK_DATA: GraphemeClusterBreakData = {
        read("data/ucd/GraphemeBreakProperty.txt")
            .parse()
            .expect("Failed parsing source data")
    };
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphemeClusterBreakData {
    pub map: BTreeMap<char, String>,
}

impl FromStr for GraphemeClusterBreakData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut map = BTreeMap::default();

        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^                     # every line
                  ([[:xdigit:]]{4,6})        # range start
                  (?:..([[:xdigit:]]{4,6}))? # range end (option)
                  [[:blank:]]*;[[:blank:]]*  # separator
                  ([[:word:]]+)              # value
                ",
            )
            .expect("Bad regex");
        }

        for capture in REGEX.captures_iter(str) {
            let start = u32::from_str_radix(&capture[1], 16).unwrap();
            let end = capture
                .get(2)
                .map_or(start, |m| u32::from_str_radix(m.as_str(), 16).unwrap());

            for point in start..(end + 1) {
                if let Some(chr) = char::from_u32(point) {
                    map.insert(chr, capture[3].to_owned());
                }
            }
        }

        Ok(GraphemeClusterBreakData { map })
    }
}
