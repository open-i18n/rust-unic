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

use itertools::Itertools;

use source::utils::read;

use regex::Regex;

lazy_static! {
    pub static ref GRAPHEME_BREAK_TESTS: GraphemeBreakTests =
        { read("data/ucd/test/GraphemeBreakTest.txt").parse().unwrap() };
}

pub struct GraphemeBreakTests {
    pub entries: Vec<GraphemeBreakTest>,
}

/// Represents a Test Case, containing a sequence of characters and GCB for each character, and for
/// each pair of adjacent chars, if they can break or not, and which rule matching the position.
///
/// Invariants:
/// ```
/// char_gcbs.len() == chars.len()
/// breaks.len() == chars.len() - 1
/// rules.len() == chars.len() - 1
/// ```
#[derive(Debug)]
pub struct GraphemeBreakTest {
    pub chars: Vec<char>,
    pub char_gcbs: Vec<String>,
    pub breaks: Vec<bool>,
    pub rules: Vec<String>,
}

impl FromStr for GraphemeBreakTests {
    type Err = ();

    fn from_str(string: &str) -> Result<GraphemeBreakTests, ()> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(
                r"(?xm)^\s*
                    ÷ \s+                 # source begin
                    (\w.*\w)              # source captured
                    \s+ ÷ \s*             # source end
                    \#                    # delimiter
                    \s* ÷ \s+ \[0\.2\]    # comment begin
                    (.*)                  # comment captured
                \s*$"
            )
            .unwrap();
            static ref COMMENT_RE: Regex = Regex::new(
                r"(?x)
                    \s+ [ \w\s<>\(\) -]+     # char name
                    \s+ \( ( \w+ ) \)       # char gcb
                    \s+ ( [÷×] )            # break opportunity or not
                    \s+ \[ ( [^\]]+ ) \]    # rule id
                "
            )
            .unwrap();
        }

        let entries = LINE_RE
            .captures_iter(string)
            .filter_map(|line| {
                let source_items: Vec<&str> =
                    line[1].split_whitespace().map(|s| s.trim()).collect();

                let codepoints: Vec<u32> = source_items
                    .iter()
                    .step(2)
                    .map(|&s| u32::from_str_radix(s, 16).expect("Bad number"))
                    .collect();
                let chars: Vec<char> = codepoints
                    .iter()
                    .filter_map(|&u| char::from_u32(u))
                    .collect();
                // Skip if any surrogate or invalid codepoints are present
                if codepoints.len() != chars.len() {
                    return None;
                }
                assert_eq!(chars.len() * 2, source_items.len() + 1);

                let breaks: Vec<bool> = source_items
                    .iter()
                    .dropping(1)
                    .step(2)
                    .map(|s| match *s {
                        "÷" => true,
                        "×" => false,
                        t => panic!("Invalid token: {:?}", t),
                    })
                    .collect();
                assert_eq!(breaks.len(), chars.len() - 1);

                let comment_items_captured = COMMENT_RE.captures_iter(&line[2]).collect::<Vec<_>>();
                let comment_items_mapped = comment_items_captured
                    .iter()
                    .map(|ref c| [&c[1], &c[2], &c[3]])
                    .collect::<Vec<_>>();
                let comment_items_flattened = comment_items_mapped
                    .iter()
                    .flat_map(|x| x.iter())
                    .collect::<Vec<_>>();
                let comment_items = &comment_items_flattened[..comment_items_flattened.len() - 2];
                assert_eq!(comment_items.len(), chars.len() * 3 - 2);

                let char_gcbs: Vec<String> = comment_items
                    .iter()
                    .step(3)
                    .map(|&s| s.to_string())
                    .collect();
                assert_eq!(char_gcbs.len(), chars.len());

                let rules: Vec<String> = comment_items
                    .iter()
                    .dropping(2)
                    .step(3)
                    .map(|&s| s.to_string())
                    .collect();
                assert_eq!(rules.len(), chars.len() - 1);

                Some(GraphemeBreakTest {
                    chars,
                    char_gcbs,
                    breaks,
                    rules,
                })
            })
            .collect();

        Ok(GraphemeBreakTests { entries })
    }
}
