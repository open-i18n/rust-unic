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
    pub static ref AGE_DATA: AgeData = { read("data/ucd/DerivedAge.txt").parse().unwrap() };
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AgeData {
    pub map: BTreeMap<char, String>,
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
            )
            .unwrap();
        }

        let mut map = BTreeMap::default();
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
                    map.insert(
                        char,
                        format!(
                            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
                            major, minor, micro,
                        ),
                    );
                }
            }
        }

        Ok(AgeData { map })
    }
}
