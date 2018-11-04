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
use std::str::FromStr;

use source::utils::read;

use regex::Regex;

lazy_static! {
    pub static ref NORMALIZATION_TESTS: NormalizationTests =
        { read("data/ucd/test/NormalizationTest.txt").parse().unwrap() };
}

pub struct NormalizationTests {
    pub entries: Box<[NormalizationTest]>,
}

pub struct NormalizationTest {
    pub source: Box<[char]>,
    pub nfc: Box<[char]>,
    pub nfd: Box<[char]>,
    pub nfkc: Box<[char]>,
    pub nfkd: Box<[char]>,
}

impl FromStr for NormalizationTests {
    type Err = ();

    fn from_str(str: &str) -> Result<NormalizationTests, ()> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # source
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfc
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfd
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfkc
                  ([[:xdigit:]]{4,6} (?:\x20[[:xdigit:]]{4,6})*); # nfkd
                  \x20
                \#"
            )
            .unwrap();
        }

        // estimate of number of test cases
        let mut tests = Vec::with_capacity(0x5000);

        for capture in REGEX.captures_iter(str) {
            tests.push(NormalizationTest {
                source: capture[1]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfc: capture[2]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfd: capture[3]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfkc: capture[4]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                nfkd: capture[5]
                    .split_whitespace()
                    .map(|s| u32::from_str_radix(s, 16).unwrap())
                    .map(|u| char::from_u32(u).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            })
        }

        Ok(NormalizationTests {
            entries: tests.into_boxed_slice(),
        })
    }
}
