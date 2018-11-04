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
    pub static ref BLOCKS_DATA: BlocksData = { read("data/ucd/Blocks.txt").parse().unwrap() };
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlocksData {
    pub map: BTreeMap<char, String>,
}

impl FromStr for BlocksData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut map = BTreeMap::default();

        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^                     # every line
                  ([[:xdigit:]]{4,6})        # range start
                  ..                         # separator
                  ([[:xdigit:]]{4,6})        # range end
                  [[:blank:]]*;[[:blank:]]*  # separator
                  (.*)                       # block name
                ",
            )
            .expect("Bad regex");
        }

        for capture in REGEX.captures_iter(str) {
            let start = u32::from_str_radix(&capture[1], 16).unwrap();
            let end = u32::from_str_radix(&capture[2], 16).unwrap();

            for point in start..(end + 1) {
                if let Some(chr) = char::from_u32(point) {
                    map.insert(chr, capture[3].to_owned());
                }
            }
        }

        Ok(BlocksData { map })
    }
}
