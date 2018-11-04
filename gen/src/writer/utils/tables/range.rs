// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::fmt::{self, Write};

use super::DisplayWrapper;

/// Create the source for a `CharDataTable`, using `CharRange`s to deduplicate data.
pub trait ToRangeCharTable<T: Eq> {
    /// Convert this mapping to a `String`.
    fn to_range_char_table<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result;
}

impl<T: Eq> ToRangeCharTable<T> for BTreeMap<char, T> {
    fn to_range_char_table<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result,
    {
        let mut entries = self.iter();
        let mut out = String::from("CharDataTable::Range(&[\n");

        if let Some((&low, mut value)) = entries.next() {
            let (mut low, mut high) = (low, low);

            let append_entry = |out: &mut String, low: char, high: char, c: &T| {
                writeln!(
                    out,
                    "    (chars!('{}'..='{}'), {}),",
                    low.escape_unicode(),
                    high.escape_unicode(),
                    DisplayWrapper(c, &display_fn),
                )
                .expect("`String` `Write` failed");
            };

            for (&char, property) in entries {
                if property != value || (char as u32) > (high as u32 + 1) {
                    append_entry(&mut out, low, high, value);
                    low = char;
                    high = char;
                    value = property;
                } else {
                    assert_eq!(char as u32, high as u32 + 1);
                    high = char;
                }
            }

            append_entry(&mut out, low, high, value);
        }

        out.push_str("])");
        out
    }
}

#[cfg(test)]
mod test {
    use super::ToRangeCharTable;
    use std::collections::BTreeMap;
    use std::fmt::Display;

    #[test]
    fn simple_range_bsearch_map() {
        let mut map: BTreeMap<char, &'static str> = Default::default();
        map.insert('a', "Low");
        map.insert('b', "Low");
        map.insert('c', "Low");
        map.insert('d', "Mid");
        map.insert('y', "High");
        map.insert('f', "Mid");
        map.insert('e', "Mid");
        map.insert('x', "High");
        map.insert('z', "High");
        assert_eq!(
            map.to_range_char_table(Display::fmt),
            "\
CharDataTable::Range(&[
    (chars!('\\u{61}'..='\\u{63}'), Low),
    (chars!('\\u{64}'..='\\u{66}'), Mid),
    (chars!('\\u{78}'..='\\u{7a}'), High),
])"
        );
    }
}
