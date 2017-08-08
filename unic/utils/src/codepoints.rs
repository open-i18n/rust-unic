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

#[inline]
#[allow(unsafe_code)]
/// Create an iterator over all characters
pub fn iter_all_chars() -> Box<DoubleEndedIterator<Item = char>> {
    Box::new(
        (CODEPOINTS_RANGE.start..SURROGATE_RANGE.start)
            .chain(SURROGATE_RANGE.end..CODEPOINTS_RANGE.end)
            .map(|codepoint| {
                debug_assert!(char::from_u32(codepoint).is_some());
                unsafe { char::from_u32_unchecked(codepoint) }
            }),
    )
}

#[cfg(test)]
mod test {
    use super::iter_all_chars;
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
        let ours = iter_all_chars().rev();
        let evident = evident_iter_all_chars()
            .collect::<Vec<_>>()
            .into_iter()
            .rev();

        assert_equal_contents(ours, evident);
    }
}
