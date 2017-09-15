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

use reader::utils::read;


lazy_static! {
    pub static ref UNICODE_VERSION: UnicodeVersion = {
        read("data/idna/ReadMe.txt").parse().unwrap()
    };
}


pub struct UnicodeVersion(
    pub u16, // major
    pub u16, // minor
    pub u16, // micro
);

impl FromStr for UnicodeVersion {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"for Version (\d+).(\d+).(\d+)").unwrap();
        }

        REGEX
            .captures(str)
            .map(|m| {
                UnicodeVersion(
                    m[1].parse().unwrap(),
                    m[2].parse().unwrap(),
                    m[3].parse().unwrap(),
                )
            })
            .ok_or(())
    }
}
