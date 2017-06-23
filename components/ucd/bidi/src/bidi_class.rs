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

use std::cmp::Ordering;


/// Represents the Unicode character
/// [*Bidi_Class*](http://www.unicode.org/reports/tr44/#Bidi_Class) property, also known as the
/// *bidirectional character type*.
///
/// * <http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types>
/// * <http://www.unicode.org/reports/tr44/#Bidi_Class_Values>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum BidiClass {
    AL,
    AN,
    B,
    BN,
    CS,
    EN,
    ES,
    ET,
    FSI,
    L,
    LRE,
    LRI,
    LRO,
    NSM,
    ON,
    PDF,
    PDI,
    R,
    RLE,
    RLI,
    RLO,
    S,
    WS,
    // [UNIC_UPDATE_ON_UNICODE_UPDATE] Source: `tables/bidi_class_type.rsv`
}


use self::BidiClass::*;

const BIDI_CLASS_TABLE: &'static [(char, char, BidiClass)] =
    include!("tables/bidi_class_values.rsv");


/// Represents **Category** of Unicode character `Bidi_Class` property, as demostrated under "Table
/// 4. Bidirectional Character Types".
///
/// * <http://www.unicode.org/reports/tr9/#Table_Bidirectional_Character_Types>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BidiClassCategory {
    /// Left-to-right and right-to-left types.
    Strong,

    /// Types associated with numbers.
    Weak,

    /// Directional formatting characters.
    ExplicitFormatting,

    /// Everything else.
    Neutral,
}

use self::BidiClassCategory::*;

impl BidiClass {
    /// Find the BidiClass of a single char.
    pub fn of(ch: char) -> BidiClass {
        bsearch_range_value_table(ch, BIDI_CLASS_TABLE)
    }

    /// If the `BidiClass` has strong or explicit Left-to-Right direction.
    #[inline]
    pub fn category(self) -> BidiClassCategory {
        match self {
            L | R | AL => Strong,
            EN | ES | ET | AN | CS | NSM | BN => Weak,
            B | S | WS | ON => Neutral,
            LRE | LRO | RLE | RLO | PDF | LRI | RLI | FSI | PDI => ExplicitFormatting,
        }
    }

    /// If the `BidiClass` has strong or explicit Left-to-Right direction.
    #[inline]
    pub fn is_ltr(self) -> bool {
        match self {
            L | LRE | LRO | LRI => true,
            _ => false,
        }
    }

    /// If the `BidiClass` has strong or explicit Right-To-Left direction.
    #[inline]
    pub fn is_rtl(self) -> bool {
        match self {
            AL | R | RLE | RLO | RLI => true,
            _ => false,
        }
    }
}

fn bsearch_range_value_table(c: char, r: &'static [(char, char, BidiClass)]) -> BidiClass {
    match r.binary_search_by(|&(lo, hi, _)| if lo <= c && c <= hi {
        Ordering::Equal
    } else if hi < c {
        Ordering::Less
    } else {
        Ordering::Greater
    }) {
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
    use std::char;

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
                (0x0860, AL),
                (0x0870, R),
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
            assert_eq!(BidiClass::of(char::from_u32(input).unwrap()), expected);
        }
    }
}
