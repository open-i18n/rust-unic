// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;
use std::fmt::Write;

/// Create the source for a `CharDataTable` mapping to `()` for set inclusion of `char`.
pub trait ToRangeCharSet {
    /// Convert this set to a `String`
    fn to_range_char_set(&self) -> String;
}

impl ToRangeCharSet for BTreeSet<char> {
    fn to_range_char_set(&self) -> String {
        let mut range_map = vec![];

        let mut entries = self.iter();
        if let Some(&low) = entries.next() {
            let (mut low, mut high) = (low, low);

            for &codepoint in entries {
                if (codepoint as u32) > (high as u32 + 1) {
                    range_map.push((low, high));
                    low = codepoint;
                    high = codepoint;
                } else {
                    assert_eq!(codepoint as u32, high as u32 + 1);
                    high = codepoint;
                }
            }

            range_map.push((low, high));
        }

        let mut out = String::from("CharRangeMap {\n");

        out.push_str("    ranges: &[\n");
        for &(low, high) in range_map.iter() {
            writeln!(
                out,
                "        chars!('{}'..='{}'),",
                low.escape_unicode(),
                high.escape_unicode(),
            ).expect("`String` `Write` failed");
        }
        out.push_str("    ],\n");

        out.push_str("    values: &[\n");
        for _ in range_map.iter() {
            out.push_str("        (),\n");
        }
        out.push_str("    ],\n");

        out.push_str("}");
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_bsearch_set() {
        let mut set: BTreeSet<char> = Default::default();
        set.insert('a');
        set.insert('b');
        set.insert('c');
        set.insert('d');
        set.insert('y');
        set.insert('f');
        set.insert('e');
        set.insert('x');
        set.insert('z');
        assert_eq!(
            set.to_range_char_set(),
            "\
CharRangeMap {
    ranges: &[
        chars!('\\u{61}'..='\\u{66}'),
        chars!('\\u{78}'..='\\u{7a}'),
    ],
    values: &[
        (),
        (),
    ],
}"
        );
    }
}
