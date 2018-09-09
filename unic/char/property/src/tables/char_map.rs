// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::slice::Iter as SliceIter;

/// A direct mapping from characters to associated data.
///
/// For the set case, use `()` as the associated value.
///
/// The members of this struct are public only so that they can be constructed
/// in const context by the macros in this crate. They should not be considered
/// stable and may be made private in the future.
// TODO(FUTURE_RUST): const constructors
#[derive(Copy, Clone, Debug)]
pub struct CharMap<V: Copy + 'static> {
    #[doc(hidden)]
    pub chars: &'static [char],
    #[doc(hidden)]
    pub values: &'static [V],
}

impl<V: Copy + 'static> Default for CharMap<V> {
    fn default() -> Self {
        CharMap {
            chars: &[],
            values: &[],
        }
    }
}

impl<V: Copy + 'static> CharMap<V> {
    /// Does this map contain a mapping for this codepoint?
    pub fn contains(&self, needle: char) -> bool {
        self.chars.binary_search(&needle).is_ok()
    }

    /// Find the associated value for a given codepoint.
    pub fn find(&self, needle: char) -> Option<V> {
        self.chars
            .binary_search(&needle)
            .map(|idx| self.values[idx])
            .ok()
    }

    /// Iterate over the `(char, V)` mappings in this map.
    pub fn iter(&self) -> Iter<V> {
        Iter {
            chars: self.chars.iter(),
            values: self.values.iter(),
        }
    }
}

impl<'a, V: Copy + 'static> IntoIterator for CharMap<V> {
    type Item = (char, V);
    type IntoIter = Iter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the `(char, V)` mappings in a `CharMap`.
#[derive(Clone, Debug)]
pub struct Iter<V: Copy + 'static> {
    chars: SliceIter<'static, char>,
    values: SliceIter<'static, V>,
}

impl<V: Copy + 'static> Iterator for Iter<V> {
    type Item = (char, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.chars
            .next()
            .map(|&ch| (ch, *self.values.next().unwrap()))
    }
}
