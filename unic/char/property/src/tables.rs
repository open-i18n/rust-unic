// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Character data tables used in UNIC.

use unic_char_range::CharRange;

pub trait TCharDataTable {
    type Item;
    fn contains(&self, needle: char) -> bool;
    fn find(&self, needle: char) -> Option<Self::Item>;
}

pub struct CharDataTableDirect<V: 'static>(#[doc(hidden)] pub &'static [(char, V)]);
impl<V: 'static> Default for CharDataTableDirect<V> {
    fn default() -> CharDataTableDirect<V> { CharDataTableDirect(&[]) }
}
impl<V: 'static + Copy> TCharDataTable for CharDataTableDirect<V> {
    type Item = V;
    fn contains(&self, needle: char) -> bool {
        self.0.binary_search_by_key(&needle, |&(k, _)| k).is_ok()
    }
    fn find(&self, needle: char) -> Option<V> {
        self.0
            .binary_search_by_key(&needle, |&(k, _)| k)
            .map(|idx| self.0[idx].1)
            .ok()
    }
}

pub struct CharDataTableRange<V: 'static>(#[doc(hidden)] pub &'static [(CharRange, V)]);
impl<V: 'static> Default for CharDataTableRange<V> {
    fn default() -> CharDataTableRange<V> { CharDataTableRange(&[]) }
}
impl<V: 'static + Copy> TCharDataTable for CharDataTableRange<V> {
    type Item = V;
    fn contains(&self, needle: char) -> bool {
        self.0
            .binary_search_by(|&(range, _)| range.cmp(needle))
            .is_ok()
    }
    fn find(&self, needle: char) -> Option<V> {
        self.0
            .binary_search_by(|&(range, _)| range.cmp(needle))
            .map(|idx| self.0[idx].1)
            .ok()
    }
}

/// A mapping from characters to some associated data.
///
/// For the set case, use `()` as the associated value.
#[derive(Copy, Clone, Debug)]
pub enum CharDataTable<V: 'static> {
    #[doc(hidden)]
    Direct(&'static [(char, V)]),
    #[doc(hidden)]
    Range(&'static [(CharRange, V)]),
}

impl<V> Default for CharDataTable<V> {
    fn default() -> Self {
        CharDataTable::Direct(&[])
    }
}

impl<V> CharDataTable<V> {
    /// Does this table contain a mapping for a character?
    pub fn contains_impl(&self, needle: char) -> bool {
        match *self {
            CharDataTable::Direct(table) => {
                table.binary_search_by_key(&needle, |&(k, _)| k).is_ok()
            }
            CharDataTable::Range(table) => table
                .binary_search_by(|&(range, _)| range.cmp(needle))
                .is_ok(),
        }
    }
}

impl<V: Copy> CharDataTable<V> {
    /// Find the associated data for a character in this table.
    pub fn find_impl(&self, needle: char) -> Option<V> {
        match *self {
            CharDataTable::Direct(table) => table
                .binary_search_by_key(&needle, |&(k, _)| k)
                .map(|idx| table[idx].1)
                .ok(),
            CharDataTable::Range(table) => table
                .binary_search_by(|&(range, _)| range.cmp(needle))
                .map(|idx| table[idx].1)
                .ok(),
        }
    }
}

impl<V: Copy> TCharDataTable for CharDataTable<V> {
    type Item = V;
    fn contains(&self, needle: char) -> bool {
        self.contains_impl(needle)
    }
    fn find(&self, needle: char) -> Option<Self::Item> {
        self.find_impl(needle)
    }
}

/// Iterator for `CharDataTable`. Iterates over pairs `(CharRange, V)`.
#[derive(Debug)]
pub struct CharDataTableIter<'a, V: 'static>(&'a CharDataTable<V>, usize);

impl<'a, V: Copy> Iterator for CharDataTableIter<'a, V> {
    type Item = (CharRange, V);

    fn next(&mut self) -> Option<Self::Item> {
        match *self.0 {
            CharDataTable::Direct(arr) => if self.1 >= arr.len() {
                None
            } else {
                let idx = self.1;
                self.1 += 1;
                let (ch, v) = arr[idx];
                Some((chars!(ch..=ch), v))
            },
            CharDataTable::Range(arr) => if self.1 >= arr.len() {
                None
            } else {
                let idx = self.1;
                self.1 += 1;
                Some(arr[idx])
            },
        }
    }
}

impl<V> CharDataTable<V> {
    /// Iterate over the entries in this table. Yields pairs `(CharRange, V)`.
    pub fn iter(&self) -> CharDataTableIter<V> {
        CharDataTableIter(self, 0)
    }
}
