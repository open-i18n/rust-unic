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

#[cfg(not(feature = "unic-ucd-category"))]
mod mark {
    use std::cmp::Ordering;

    const GENERAL_CATEGORY_MARK: &'static [(char, char)] =
        include!("tables/general_category_mark.rsv");

    /// Return whether the given character is a combining mark (`General_Category=Mark`)
    pub fn is_combining_mark(c: char) -> bool {
        bsearch_range_table(c, GENERAL_CATEGORY_MARK)
    }

    fn bsearch_range_table(c: char, r: &'static [(char, char)]) -> bool {
        r.binary_search_by(|&(lo, hi)| if lo <= c && c <= hi {
            Ordering::Equal
        } else if hi < c {
            Ordering::Less
        } else {
            Ordering::Greater
        }).is_ok()
    }
}

#[cfg(feature = "unic-ucd-category")]
mod mark {
    extern crate unic_ucd_category;
    use self::unic_ucd_category::GeneralCategory;

    /// Return whether the given character is a combining mark (`General_Category=Mark`)
    pub fn is_combining_mark(c: char) -> bool {
        GeneralCategory::of(c).is_mark()
    }
}

pub use self::mark::is_combining_mark;

#[cfg(test)]
mod tests {
    use std::char;

    use super::*;

    #[test]
    fn test_is_combining_mark_ascii() {
        for cp in 0..0x7f {
            assert!(!is_combining_mark(char::from_u32(cp).unwrap()));
        }
    }

    // TODO: Add more tests for edge cases, Hangul comp/decomp, etc

    #[test]
    fn test_is_combining_mark_misc() {
        // https://github.com/unicode-rs/unicode-normalization/issues/16
        // U+11C3A BHAIKSUKI VOWEL SIGN O
        // Category: Mark, Nonspacing [Mn]
        assert!(is_combining_mark('\u{11C3A}'));

        // U+11C3F BHAIKSUKI SIGN VIRAMA
        // Category: Mark, Nonspacing [Mn]
        assert!(is_combining_mark('\u{11C3F}'));
    }
}
