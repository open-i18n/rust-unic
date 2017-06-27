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


//! Accessor for Canonical_Combining_Class (ccc) property

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
    fn ccc_is_not_reordered(&self) -> bool;
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
    fn ccc_is_not_reordered(&self) -> bool {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        assert_eq!(CanonicalCombiningClass::of('\u{0000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{0040}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{0041}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{0062}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{007F}'), 0);
    }

    #[test]
    fn test_bmp() {
        // Combining Diacritical Marks
        assert_eq!(CanonicalCombiningClass::of('\u{0300}'), 230);
        assert_eq!(CanonicalCombiningClass::of('\u{0314}'), 230);
        assert_eq!(CanonicalCombiningClass::of('\u{0315}'), 232);
        assert_eq!(CanonicalCombiningClass::of('\u{0316}'), 220);
        assert_eq!(CanonicalCombiningClass::of('\u{0319}'), 220);

        // Hebrew
        assert_eq!(CanonicalCombiningClass::of('\u{0590}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{05D0}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{05D1}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{05FF}'), 0);

        // Arabic
        assert_eq!(CanonicalCombiningClass::of('\u{0600}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{0627}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{064B}'), 27);
        assert_eq!(CanonicalCombiningClass::of('\u{064C}'), 28);
        assert_eq!(CanonicalCombiningClass::of('\u{064D}'), 29);
        assert_eq!(CanonicalCombiningClass::of('\u{064E}'), 30);
        assert_eq!(CanonicalCombiningClass::of('\u{064F}'), 31);
        assert_eq!(CanonicalCombiningClass::of('\u{0650}'), 32);
        assert_eq!(CanonicalCombiningClass::of('\u{0651}'), 33);
        assert_eq!(CanonicalCombiningClass::of('\u{0652}'), 34);

        assert_eq!(CanonicalCombiningClass::of('\u{07BF}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{07C0}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{085F}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{0860}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{0870}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{089F}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{08A0}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{089F}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{08FF}'), 230);

        //  Currency Symbols
        assert_eq!(CanonicalCombiningClass::of('\u{20A0}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{20CF}'), 0);

        // Arabic Presentation Forms
        assert_eq!(CanonicalCombiningClass::of('\u{FB1D}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FB4F}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FB50}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FDCF}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FDF0}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FDFF}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FE70}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FEFE}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FEFF}'), 0);

        // noncharacters
        assert_eq!(CanonicalCombiningClass::of('\u{FDD0}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FDD1}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FDEE}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FDEF}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FFFE}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{FFFF}'), 0);
    }

    #[test]
    fn test_smp() {
        assert_eq!(CanonicalCombiningClass::of('\u{10000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{101fc}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{101fd}'), 220);
        assert_eq!(CanonicalCombiningClass::of('\u{101fe}'), 0);

        assert_eq!(CanonicalCombiningClass::of('\u{1e000}'), 230);

        assert_eq!(CanonicalCombiningClass::of('\u{1e949}'), 230);
        assert_eq!(CanonicalCombiningClass::of('\u{1e94a}'), 7);
        assert_eq!(CanonicalCombiningClass::of('\u{1e94b}'), 0);

        assert_eq!(CanonicalCombiningClass::of('\u{1efff}'), 0);

        // noncharacters
        assert_eq!(CanonicalCombiningClass::of('\u{1fffe}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{1ffff}'), 0);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(CanonicalCombiningClass::of('\u{30000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{40000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{50000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{60000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{70000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{80000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{90000}'), 0);
        assert_eq!(CanonicalCombiningClass::of('\u{a0000}'), 0);
    }
}
