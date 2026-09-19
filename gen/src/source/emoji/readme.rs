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

use crate::source::utils::read;

pub struct EmojiDataVersion {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

lazy_static! {
    pub static ref EMOJI_VERSION: EmojiDataVersion = {
        read("external/unicode/emoji/data/ReadMe.txt")
            .parse()
            .unwrap()
    };
}

impl FromStr for EmojiDataVersion {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?:Unicode Emoji, Version|version) (\d+)\.(\d+)(?:\.\d+)? of\s+UTS #51|Unicode Emoji, Version (\d+)\.(\d+)"
            )
            .unwrap();
        }

        REGEX
            .captures(str)
            .map(|m| {
                let (major, minor) = if m.get(1).is_some() {
                    (&m[1], &m[2])
                } else {
                    (&m[3], &m[4])
                };
                EmojiDataVersion {
                    major: major.parse().unwrap(),
                    minor: minor.parse().unwrap(),
                    micro: 0,
                }
            })
            .ok_or(())
    }
}
