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

/// A simple binary search array slice.
///
/// Output format:
///
/// ```text
/// &[
///     ('char', Value),
///     ('char', Value),
/// ]
/// ```
///
/// Where
///
/// - `'char'` is a `char::escape_unicode` literal for the character
/// - `Value` is the result of running `display_fn` over the associated value
///
/// It is guaranteed that the `'char'` of one entry will always be ordered before the `'char'` of
/// the next range (such that the array slice is fit for a binary search).
pub trait ToSingleBSearchMap<T> {
    /// Convert this mapping to a `String`.
    fn to_single_bsearch_map<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result;
}

impl<T> ToSingleBSearchMap<T> for BTreeMap<char, T> {
    fn to_single_bsearch_map<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result,
    {
        let entries = self.iter();
        let mut out = String::from("&[\n");

        for (char, property) in entries {
            out.push_str(&format!(
                "    ('{}', {}),\n",
                char.escape_unicode(),
                DisplayWrapper(property, &display_fn),
            ));
        }

        out.push_str("]");
        out
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::fmt::Display;
    use super::ToSingleBSearchMap;

    #[test]
    fn simple_single_bsearch_map() {
        let mut map: BTreeMap<char, &'static str> = Default::default();
        map.insert('a', "A");
        map.insert('b', "B");
        map.insert('c', "C");
        map.insert('z', "Z");
        map.insert('y', "Y");
        map.insert('x', "X");
        assert_eq!(
            map.to_single_bsearch_map(Display::fmt),
            "\
&[
    ('\\u{61}', A),
    ('\\u{62}', B),
    ('\\u{63}', C),
    ('\\u{78}', X),
    ('\\u{79}', Y),
    ('\\u{7a}', Z),
]"
        );
    }
}
