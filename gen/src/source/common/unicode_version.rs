// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

use regex::Regex;

pub struct UnicodeVersion {
    pub major: u16,
    pub minor: u16,
    pub micro: u16,
}

impl FromStr for UnicodeVersion {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"for (?:Version|version) (\d+).(\d+).(\d+)").unwrap();
        }

        REGEX
            .captures(str)
            .map(|m| UnicodeVersion {
                major: m[1].parse().unwrap(),
                minor: m[2].parse().unwrap(),
                micro: m[3].parse().unwrap(),
            })
            .ok_or(())
    }
}
