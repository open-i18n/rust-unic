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
        let mut entries = self.iter();
        let mut out = String::from("CharDataTable::Range(&[\n");

        if let Some(&low) = entries.next() {
            let (mut low, mut high) = (low, low);

            let append_entry = |out: &mut String, low: char, high: char| {
                writeln!(
                    out,
                    "    (chars!('{}'..='{}'), ()),",
                    low.escape_unicode(),
                    high.escape_unicode(),
                )
                .expect("`String` `Write` failed");
            };

            for &char in entries {
                if (char as u32) > (high as u32 + 1) {
                    append_entry(&mut out, low, high);
                    low = char;
                    high = char;
                } else {
                    assert_eq!(char as u32, high as u32 + 1);
                    high = char;
                }
            }

            append_entry(&mut out, low, high);
        }

        out.push_str("])");
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
CharDataTable::Range(&[
    (chars!('\\u{61}'..='\\u{66}'), ()),
    (chars!('\\u{78}'..='\\u{7a}'), ()),
])"
        );
    }
}
