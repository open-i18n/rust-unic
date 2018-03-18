// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::iter::Peekable;
use core::slice::Iter as SliceIter;

use unic_char_range::{CharIter, CharRange};

/// A mapping from ranges of characters to associated data.
///
/// For the set case, use `()` as the associated value.
///
/// The members of this struct are public only so that they can be constructed
/// in const context by the macros in this crate. They should not be considered
/// stable and may be made private in the future.
// TODO(FUTURE_RUST): const constructors
#[derive(Copy, Clone, Debug)]
pub struct CharRangeMap<V: Copy + 'static> {
    #[doc(hidden)]
    pub ranges: &'static [CharRange],
    #[doc(hidden)]
    pub values: &'static [V],
}

impl<V: Copy + 'static> CharRangeMap<V> {
    /// Does this map contain a mapping for this codepoint?
    pub fn contains(&self, needle: char) -> bool {
        self.ranges
            .binary_search_by(|range| range.cmp_char(needle))
            .is_ok()
    }

    /// Find the associated value for a given codepoint.
    pub fn find(&self, needle: char) -> Option<V> {
        self.ranges
            .binary_search_by(|range| range.cmp_char(needle))
            .map(|idx| self.values[idx])
            .ok()
    }

    /// Iterate over the `(char, V)` mappings in this map.
    pub fn iter(&self) -> Iter<V> {
        let mut range_iter = self.ranges.iter();
        Iter {
            current: range_iter.next().map(CharRange::iter),
            ranges: range_iter,
            values: self.values.iter().peekable(),
        }
    }
}

/// Iterator over the `(char, V)` mappings in a `CharRangeMap`.
#[derive(Clone, Debug)]
pub struct Iter<V: Copy + 'static> {
    current: Option<CharIter>,
    ranges: SliceIter<'static, CharRange>,
    values: Peekable<SliceIter<'static, V>>,
}

impl<V: Copy + 'static> Iterator for Iter<V> {
    type Item = (char, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut current) = self.current.take() {
            match current.next() {
                Some(c) => {
                    self.current = Some(current);
                    Some((
                        c,
                        **self.values.peek().expect("CharRangeMap Iter unreachable"),
                    ))
                }
                None => {
                    self.current = self.ranges.next().map(CharRange::iter);
                    self.values.next();
                    self.next()
                }
            }
        } else {
            None
        }
    }
}
