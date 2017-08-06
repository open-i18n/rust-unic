// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! # UNIC — Utils — Code Points
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Utilities for working with Unicode Code Points
//!
//! ## Definitions
//!
//! * [**Unicode Code Point**](http://unicode.org/glossary/#code_point)
//! * [**Unicode Scalar Value**](http://unicode.org/glossary/#unicode_scalar_value)

use std::char;
use std::ops::Range;


/// Range of Unicode Code Points.
///
/// Reference: <http://unicode.org/glossary/#code_point>
pub const CODEPOINTS_RANGE: Range<u32> = 0x0..(0x10_FFFF + 1);

/// Range of Surrogate Code Points.
///
/// Reference: <http://unicode.org/glossary/#surrogate_code_point>
pub const SURROGATE_RANGE: Range<u32> = 0xD800..(0xDFFF + 1);

/// Check a code-point against `SURROGATE_CODEPOINTS_RANGE`.
#[inline]
pub fn iter_all_chars() -> Box<Iterator<Item = char>> {
    Box::new(CharIterator::new())
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
/// An iterator over all `char`.
pub struct CharIterator {
    /// The codepoint of the smallest uniterated codepoint.
    ///
    /// If greater than or equal to `end`, iteration is finished.
    ///
    /// # Safety
    ///
    /// Must be a valid, non-surrogate codepoint.
    current: u32,

    /// The codepoint one greater than the largest uniterated codepoint.
    ///
    /// If less than or equal to `current`, iteration is finished.
    ///
    /// # Safety
    ///
    /// Must be a valid, non-surrogate codepoint.
    end: u32,
}

impl CharIterator {
    fn new() -> CharIterator {
        CharIterator {
            current: CODEPOINTS_RANGE.start,
            end: CODEPOINTS_RANGE.end,
        }
    }

    // cannot be is_empty because usages conflict the unstable is_empty
    fn is_finished(&self) -> bool {
        self.current >= self.end
    }
}

impl Default for CharIterator {
    fn default() -> CharIterator {
        CharIterator::new()
    }
}

impl Iterator for CharIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished() {
            return None;
        }

        let next = unsafe { char::from_u32_unchecked(self.current) };

        self.current += 1;
        if self.current == SURROGATE_RANGE.start {
            self.current = SURROGATE_RANGE.end
        }

        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for CharIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.is_finished() {
            return None;
        }

        if self.end == SURROGATE_RANGE.end {
            self.end = SURROGATE_RANGE.start;
        }
        self.end -= 1;

        let next = unsafe { char::from_u32_unchecked(self.end) };

        Some(next)
    }
}

impl ExactSizeIterator for CharIterator {
    fn len(&self) -> usize {
        let mut remaining_codepoints = self.end - self.current;
        if self.current < SURROGATE_RANGE.start && self.end >= SURROGATE_RANGE.end {
            remaining_codepoints -= SURROGATE_RANGE.len() as u32;
        }
        remaining_codepoints as usize
    }
}

#[cfg(test)]
mod test {
    use super::{iter_all_chars, CharIterator};
    use std::{char, fmt};

    fn evident_iter_all_chars() -> Box<Iterator<Item = char>> {
        Box::new((0..char::MAX as u32 + 1).filter_map(char::from_u32))
    }

    fn assert_equal_contents<I, J, Item>(lhs: I, rhs: J)
    where
        I: Iterator<Item = Item>,
        J: Iterator<Item = Item>,
        Item: PartialEq + fmt::Debug,
    {
        let mut rhs = rhs.fuse();
        for item in lhs {
            assert_eq!(Some(item), rhs.next());
        }
        assert_eq!(rhs.count(), 0);
    }

    #[test]
    fn iter_all_chars_does_so() {
        let ours = iter_all_chars();
        let evident = evident_iter_all_chars();

        assert_equal_contents(ours, evident);
    }

    #[test]
    fn reverse_iteration_works() {
        let ours = CharIterator::new().rev();
        let evident = evident_iter_all_chars()
            .collect::<Vec<_>>()
            .into_iter()
            .rev();

        assert_equal_contents(ours, evident);
    }

    #[test]
    fn is_exact_size_hint_forwards() {
        let mut iter = CharIterator::new();
        let mut len = iter.len();
        let (mut lower_bound, higher_bound) = iter.size_hint();

        assert_eq!(Some(lower_bound), higher_bound);
        let mut higher_bound = higher_bound.unwrap();

        while let Some(_) = iter.next() {
            len -= 1;
            lower_bound -= 1;
            higher_bound -= 1;

            assert_eq!(len, iter.len());
            assert_eq!((lower_bound, Some(higher_bound)), iter.size_hint());
        }

        assert_eq!(len, 0);
        assert_eq!(lower_bound, 0);
        assert_eq!(higher_bound, 0);
    }

    #[test]
    fn is_exact_size_hint_backwards() {
        let mut iter = CharIterator::new();
        let mut len = iter.len();
        let (mut lower_bound, higher_bound) = iter.size_hint();

        assert_eq!(Some(lower_bound), higher_bound);
        let mut higher_bound = higher_bound.unwrap();

        while let Some(_) = iter.next_back() {
            len -= 1;
            lower_bound -= 1;
            higher_bound -= 1;

            assert_eq!(len, iter.len());
            assert_eq!((lower_bound, Some(higher_bound)), iter.size_hint());
        }

        assert_eq!(len, 0);
        assert_eq!(lower_bound, 0);
        assert_eq!(higher_bound, 0);
    }
}
