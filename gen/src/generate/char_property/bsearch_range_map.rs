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
use std::fmt;

use super::DisplayWrapper;

/// A simple run-collapsing binary search array slice.
///
/// Output format:
///
/// ```text
/// &[
///     ('low', 'high', Value),
///     ('low', 'high', Value),
/// ]
/// ```
///
/// Where
///
/// - `'low'` is a `char::escape_unicode` literal for the lowest character in the range
/// - `'high'` is a `char::escape_unicode` literal for the highest character in the range
/// - `Value` is the result of running `display_fn` over the associated value
///
/// It is guaranteed that the `'high'` of one range will always be ordered before the `'low'` of
/// the next range (such that the array slice is fit for a binary search). The ranges
/// represented by `'low'` and `'high'` are inclusive on both ends.
pub trait ToRangeBSearchMap<T: Eq> {
    /// Convert this mapping to a `String`.
    fn to_range_bsearch_map<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result;
}

impl<T: Eq> ToRangeBSearchMap<T> for BTreeMap<char, T> {
    fn to_range_bsearch_map<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result,
    {
        let mut entries = self.iter();

        let first = entries.next();
        if first.is_none() {
            return String::from("&[]");
        }

        let mut out = String::from("&[\n");
        let (mut low, mut value) = first.unwrap();
        let mut high = low;

        let append_triple = |out: &mut String, a: char, b: char, c: &T| {
            out.push_str(&format!(
                "    ('{}', '{}', {}),\n",
                a.escape_unicode(),
                b.escape_unicode(),
                DisplayWrapper(c, &display_fn),
            ));
        };

        for (char, property) in entries {
            if property != value || (*char as u32) > (*high as u32 + 1) {
                append_triple(&mut out, *low, *high, value);
                low = char;
                high = char;
                value = property;
            } else {
                assert_eq!(*char as u32, *high as u32 + 1);
                high = char;
            }
        }

        append_triple(&mut out, *low, *high, value);
        out.push_str("]");
        out
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::fmt::Display;
    use super::ToRangeBSearchMap;

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
            map.to_range_bsearch_map(Display::fmt),
            "\
&[
    ('\\u{61}', '\\u{63}', Low),
    ('\\u{64}', '\\u{66}', Mid),
    ('\\u{78}', '\\u{7a}', High),
]"
        );
    }
}
