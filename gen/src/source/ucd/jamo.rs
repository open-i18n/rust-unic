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
    pub static ref JAMO_DATA: JamoData = { read("data/ucd/Jamo.txt").parse().unwrap() };
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JamoData {
    pub map: BTreeMap<char, String>,
}

impl FromStr for JamoData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^              # every line
                  ([[:xdigit:]]{4,6}) # codepoint
                  ;[[:blank:]]*       # separator
                  ([[:alpha:]]*)      # jamo short name
                ",
            )
            .unwrap();
        }

        let mut map = BTreeMap::default();
        for capture in REGEX.captures_iter(str) {
            let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
            let chr = char::from_u32(code_point).unwrap();
            map.insert(chr, capture[2].to_owned());
        }

        Ok(JamoData { map })
    }
}
