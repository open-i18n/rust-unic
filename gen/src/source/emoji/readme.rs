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

use source::utils::read;

pub struct EmojiDataVersion {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

lazy_static! {
    pub static ref EMOJI_VERSION: EmojiDataVersion =
        { read("data/emoji/ReadMe.txt").parse().unwrap() };
}

impl FromStr for EmojiDataVersion {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"Unicode Emoji, Version (\d+).(\d+)").unwrap();
        }

        REGEX
            .captures(str)
            .map(|m| EmojiDataVersion {
                major: m[1].parse().unwrap(),
                minor: m[2].parse().unwrap(),
                micro: 0,
            })
            .ok_or(())
    }
}
