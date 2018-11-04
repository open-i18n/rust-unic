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
use std::collections::BTreeSet;
use std::str::FromStr;

use regex::Regex;

use source::utils::read;

lazy_static! {
    pub static ref COMPOSITION_EXCLUSIONS: CompositionExclusions = {
        read("data/ucd/DerivedNormalizationProps.txt")
            .parse()
            .unwrap()
    };
}

pub struct CompositionExclusions {
    pub set: BTreeSet<char>,
}

impl FromStr for CompositionExclusions {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^
                  ([[:xdigit:]]{4,6})
                  (?:\.\.([[:xdigit:]]{4,6}))?
                  [[:space:]]*;
                  \x20Full_Composition_Exclusion\x20
                \#",
            )
            .unwrap();
        }

        let mut exclusions = BTreeSet::default();
        for capture in REGEX.captures_iter(str) {
            let low = u32::from_str_radix(&capture[1], 16).unwrap();
            let high = capture
                .get(2)
                .map_or(low, |m| u32::from_str_radix(m.as_str(), 16).unwrap());
            for point in low..(high + 1) {
                if let Some(char) = char::from_u32(point) {
                    exclusions.insert(char);
                }
            }
        }

        Ok(CompositionExclusions { set: exclusions })
    }
}
