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


#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # UNIC — UCD — Bidi
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Accessor for `Bidi_Class` property from Unicode Character Database (UCD)


mod tables;
mod traits;

pub use tables::{BidiClass, UNICODE_VERSION};
pub use traits::{BidiChar, BidiStr};

use std::cmp::Ordering::{Equal, Less, Greater};
use std::char;

use tables::BIDI_CLASS_TABLE;
use BidiClass::*;

/// Find the BidiClass of a single char.
pub fn bidi_class(c: char) -> BidiClass {
    bsearch_range_value_table(c, BIDI_CLASS_TABLE)
}

/// If `bidi_class` is an explicit directional initiator or terminator.
#[inline]
pub fn is_explicit(bidi_class: BidiClass) -> bool {
    match bidi_class {
        FSI | LRE | LRI | LRO | NSM | PDF | PDI | RLE | RLI | RLO => true,
        _ => false,
    }
}

/// If `bidi_class` has Right-To-Left direction.
#[inline]
pub fn is_rtl(bidi_class: BidiClass) -> bool {
    match bidi_class {
        AL | R | RLE | RLO | RLI => true,
        _ => false,
    }
}

fn bsearch_range_value_table(c: char, r: &'static [(char, char, BidiClass)]) -> BidiClass {
    match r.binary_search_by(
        |&(lo, hi, _)| if lo <= c && c <= hi {
            Equal
        } else if hi < c {
            Less
        } else {
            Greater
        }
    ) {
        Ok(idx) => {
            let (_, _, cat) = r[idx];
            cat
        }
        // UCD/extracted/DerivedBidiClass.txt: "All code points not explicitly listed
        // for Bidi_Class have the value Left_To_Right (L)."
        Err(_) => L,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bidi_class() {
        for (input, expected) in
            vec![
                (0x0000, BN),
                (0x0040, ON),
                (0x0041, L),
                (0x0062, L),
                (0x007F, BN),

                // Hebrew
                (0x0590, R),
                (0x05D0, R),
                (0x05D1, R),
                (0x05FF, R),

                // Arabic
                (0x0600, AN),
                (0x0627, AL),
                (0x07BF, AL),

                // Default R + Arabic Extras
                (0x07C0, R),
                (0x085F, R),
                (0x0860, R),
                (0x089F, R),
                (0x08A0, AL),
                (0x089F, R),
                (0x08FF, NSM),

                // Default ET
                (0x20A0, ET),
                (0x20CF, ET),

                // Arabic Presentation Forms
                (0xFB1D, R),
                (0xFB4F, R),
                (0xFB50, AL),
                (0xFDCF, AL),
                (0xFDF0, AL),
                (0xFDFF, AL),
                (0xFE70, AL),
                (0xFEFE, AL),
                (0xFEFF, BN),

                // Default AL + R
                (0x10800, R),
                (0x10FFF, R),
                (0x1E800, R),
                (0x1EDFF, R),
                (0x1EE00, AL),
                (0x1EEFF, AL),
                (0x1EF00, R),
                (0x1EFFF, R),
            ] {
            assert_eq!(bidi_class(char::from_u32(input).unwrap()), expected);
        }
    }
}
