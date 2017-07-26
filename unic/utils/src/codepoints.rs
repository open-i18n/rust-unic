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

use std::char::from_u32;
use std::ops::Range;


/// Range of Unicode Code Points.
///
/// Reference: <http://unicode.org/glossary/#code_point>
pub const CODEPOINTS_RANGE: Range<u32> = 0x0..(0x10FFFF + 1);

/// Range of Unicde Scalar Values.
///
/// Reference: <http://unicode.org/glossary/#unicode_scalar_value>
const SCALAR_VALUE_RANGE_1: Range<u32> = 0x0..0xD800;
const SCALAR_VALUE_RANGE_2: Range<u32> = (0xDFFF + 1)..(0x10_FFFF + 1);

/// Check a code-point against `SURROGATE_CODEPOINTS_RANGE`.
#[inline]
pub fn iter_all_chars() -> Box<Iterator<Item = char>> {
    Box::new(
        // TODO: maybe use char::from_u32_unchecked()
        SCALAR_VALUE_RANGE_1
            .chain(SCALAR_VALUE_RANGE_2)
            .filter_map(from_u32),
    )
}
