// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! Accessor for Canonical Combining Class (ccc)

// TODO: [API] Support Canonical_Combining_Class_Values, either by creating a separate `enum` type,
// or converting `CanonicalCombiningClass` into enum and allowing access to numerical value.
// <http://unicode.org/reports/tr44/#Canonical_Combining_Class_Values>

// TODO: Use associate type `u8`

use std::cmp::Ordering;


/// Represents *Canonical_Combining_Class* property of a Unicode character.
///
/// * <http://unicode.org/reports/tr44/#Canonical_Combining_Class>
pub trait CanonicalCombiningClass {
    /// If the *ccc* has value `Not_Reordered` (`0`).
    fn is_not_reordered(&self) -> bool;
}


const CANONICAL_COMBINING_CLASS_VALUES: &'static [(char, char, u8)] =
    include!("tables/canonical_combining_class_values.rsv");


impl CanonicalCombiningClass {
    /// Lookup Canonical Combining Class of the character
    pub fn of(ch: char) -> u8 {
        bsearch_range_value_table(ch, CANONICAL_COMBINING_CLASS_VALUES)
    }
}


impl CanonicalCombiningClass for u8 {
    fn is_not_reordered(&self) -> bool {
        *self == 0
    }
}


fn bsearch_range_value_table(c: char, r: &'static [(char, char, u8)]) -> u8 {
    match r.binary_search_by(|&(lo, hi, _)| if lo <= c && c <= hi {
        Ordering::Equal
    } else if hi < c {
        Ordering::Less
    } else {
        Ordering::Greater
    }) {
        Ok(idx) => {
            let (_, _, result) = r[idx];
            result
        }
        Err(_) => 0,
    }
}
