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

/// Create the source for a `CharDataTable`, using direct mappings from char to values
pub trait ToDirectCharTable<T> {
    /// Convert this mapping to a `String`.
    fn to_direct_char_table<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result;
}

impl<T> ToDirectCharTable<T> for BTreeMap<char, T> {
    fn to_direct_char_table<F>(&self, display_fn: F) -> String
    where
        F: Fn(&T, &mut fmt::Formatter) -> fmt::Result,
    {
        let mut out = String::from("CharMap {\n");

        out.push_str("    chars: &[\n");
        for ch in self.keys() {
            writeln!(out, "        '{}',", ch.escape_unicode()).expect("`String` `Write` failed");
        }
        out.push_str("    ],\n");

        out.push_str("    values: &[\n");
        for val in self.values() {
            writeln!(out, "        {},", DisplayWrapper(val, &display_fn))
                .expect("`String` `Write` failed");
        }
        out.push_str("    ],\n");

        out.push_str("}");
        out
    }
}

#[cfg(test)]
mod test {
    use super::ToDirectCharTable;
    use std::collections::BTreeMap;
    use std::fmt::Display;

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
            map.to_direct_char_table(Display::fmt),
            "\
CharMap {
    chars: &[
        '\\u{61}',
        '\\u{62}',
        '\\u{63}',
        '\\u{78}',
        '\\u{79}',
        '\\u{7a}',
    ],
    values: &[
        A,
        B,
        C,
        X,
        Y,
        Z,
    ],
}"
        );
    }
}
