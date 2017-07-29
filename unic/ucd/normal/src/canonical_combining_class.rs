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


//! Accessor for *Canonical_Combining_Class* (ccc) property
//!
//! Reference: <http://unicode.org/reports/tr44/#Canonical_Combining_Class_Values>


use std::fmt;

use unic_utils::{CharDataTable, CharProperty, NumericCharProperty};


/// Represents *Canonical_Combining_Class* property of a Unicode character.
///
/// * <http://unicode.org/reports/tr44/#Canonical_Combining_Class>
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanonicalCombiningClass(u8);


impl CharProperty for CanonicalCombiningClass {
    fn of(ch: char) -> Self {
        Self::of(ch)
    }
}


impl NumericCharProperty<u8> for CanonicalCombiningClass {
    /// Get numeric value for character property value
    fn number(&self) -> u8 {
        self.number()
    }
}


// TODO: Once we fully adopt 1.20 change these to associated consts on CanonicalCombiningClass
/// *Canonical_Combining_Class*es by their name
#[allow(non_upper_case_globals)]
pub mod values {
    use super::CanonicalCombiningClass as CCC;

    /// Spacing and enclosing marks; also many vowel and consonant signs, even if nonspacing
    pub const NotReordered: CCC = CCC(0);
    /// Marks which overlay a base letter or symbol
    pub const Overlay: CCC = CCC(1);
    /// Diacritic nukta marks in Brahmi-derived scripts
    pub const Nukta: CCC = CCC(7);
    /// Hiragana/Katakana voicing marks
    pub const KanaVoicing: CCC = CCC(8);
    /// Viramas
    pub const Virama: CCC = CCC(9);
    /// Marks attached at the bottom left
    pub const AttatchedBelowLeft: CCC = CCC(200);
    /// Marks attached directly below
    pub const AttachedBelow: CCC = CCC(202);
    /// Marks attached at the bottom right
    pub const AttachedBelowRight: CCC = CCC(204);
    /// Marks attached to the left
    pub const AttachedLeft: CCC = CCC(208);
    /// Marks attached to the right
    pub const AttachedRight: CCC = CCC(210);
    /// Marks attached at the top left
    pub const AttachedAboveLeft: CCC = CCC(212);
    /// Marks attached directly above
    pub const AttatchedAbove: CCC = CCC(214);
    /// Marks attached at the top right
    pub const AttatchedAboveRight: CCC = CCC(216);
    /// Distinct marks at the bottom left
    pub const BelowLeft: CCC = CCC(218);
    /// Distinct marks directly below
    pub const Below: CCC = CCC(220);
    /// Distinct marks at the bottom right
    pub const BelowRight: CCC = CCC(222);
    /// Distinct marks to the left
    pub const Left: CCC = CCC(224);
    /// Distinct marks to the right
    pub const Right: CCC = CCC(226);
    /// Distinct marks at the top left
    pub const AboveLeft: CCC = CCC(228);
    /// Distinct marks directly above
    pub const Above: CCC = CCC(230);
    /// Distinct marks at the top right
    pub const AboveRight: CCC = CCC(232);
    /// Distinct marks subtending two bases
    pub const DoubleBelow: CCC = CCC(233);
    /// Distinct marks extending above two bases
    pub const DoubleAbove: CCC = CCC(234);
    /// Greek iota subscript only
    pub const IotaSubscript: CCC = CCC(240);
}


const CANONICAL_COMBINING_CLASS_VALUES: &'static [(char, char, CanonicalCombiningClass)] =
    include!("tables/canonical_combining_class_values.rsv");

impl CanonicalCombiningClass {
    /// Find the character *Canonical_Combining_Class* property value.
    pub fn of(ch: char) -> CanonicalCombiningClass {
        *CANONICAL_COMBINING_CLASS_VALUES.find_or(ch, &CanonicalCombiningClass(0))
    }

    /// Human-readable description of the property value.
    // TODO: Needs to be improved by returning long-name with underscores replaced by space.
    #[inline]
    pub fn display(&self) -> String {
        format!("{}", self.number())
    }
}

impl fmt::Display for CanonicalCombiningClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}


impl CanonicalCombiningClass {
    /// Get numeric *Canonical_Combining_Class* value
    pub fn number(&self) -> u8 {
        self.0
    }

    /// If the *ccc* has value `Not_Reordered` (`0`).
    pub fn is_not_reordered(&self) -> bool {
        self.0 == 0
    }

    /// If the *ccc* any value other than `Not_Reordered` (`0`).
    pub fn is_reordered(&self) -> bool {
        self.0 != 0
    }
}


#[cfg(test)]
mod tests {
    use super::CanonicalCombiningClass as CCC;
    use super::values as ccc;

    #[test]
    fn test_ascii() {
        assert_eq!(CCC::of('\u{0000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{0040}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{0041}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{0062}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{007F}'), ccc::NotReordered);
    }

    #[test]
    fn test_bmp() {
        // Combining Diacritical Marks
        assert_eq!(CCC::of('\u{0300}'), ccc::Above);
        assert_eq!(CCC::of('\u{0314}'), ccc::Above);
        assert_eq!(CCC::of('\u{0315}'), ccc::AboveRight);
        assert_eq!(CCC::of('\u{0316}'), ccc::Below);
        assert_eq!(CCC::of('\u{0319}'), ccc::Below);

        // Hebrew
        assert_eq!(CCC::of('\u{0590}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{05D0}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{05D1}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{05FF}'), ccc::NotReordered);

        // Arabic
        assert_eq!(CCC::of('\u{0600}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{0627}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{064B}'), CCC(27));
        assert_eq!(CCC::of('\u{064C}'), CCC(28));
        assert_eq!(CCC::of('\u{064D}'), CCC(29));
        assert_eq!(CCC::of('\u{064E}'), CCC(30));
        assert_eq!(CCC::of('\u{064F}'), CCC(31));
        assert_eq!(CCC::of('\u{0650}'), CCC(32));
        assert_eq!(CCC::of('\u{0651}'), CCC(33));
        assert_eq!(CCC::of('\u{0652}'), CCC(34));

        assert_eq!(CCC::of('\u{07BF}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{07C0}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{085F}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{0860}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{0870}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{089F}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{08A0}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{089F}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{08FF}'), ccc::Above);

        //  Currency Symbols
        assert_eq!(CCC::of('\u{20A0}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{20CF}'), ccc::NotReordered);

        // Arabic Presentation Forms
        assert_eq!(CCC::of('\u{FB1D}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FB4F}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FB50}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FDCF}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FDF0}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FDFF}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FE70}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FEFE}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FEFF}'), ccc::NotReordered);

        // noncharacters
        assert_eq!(CCC::of('\u{FDD0}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FDD1}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FDEE}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FDEF}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FFFE}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{FFFF}'), ccc::NotReordered);
    }

    #[test]
    fn test_smp() {
        assert_eq!(CCC::of('\u{10000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{101fc}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{101fd}'), ccc::Below);
        assert_eq!(CCC::of('\u{101fe}'), ccc::NotReordered);

        assert_eq!(CCC::of('\u{1e000}'), ccc::Above);

        assert_eq!(CCC::of('\u{1e949}'), ccc::Above);
        assert_eq!(CCC::of('\u{1e94a}'), CCC(7));
        assert_eq!(CCC::of('\u{1e94b}'), ccc::NotReordered);

        assert_eq!(CCC::of('\u{1efff}'), ccc::NotReordered);

        // noncharacters
        assert_eq!(CCC::of('\u{1fffe}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{1ffff}'), ccc::NotReordered);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(CCC::of('\u{30000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{40000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{50000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{60000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{70000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{80000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{90000}'), ccc::NotReordered);
        assert_eq!(CCC::of('\u{a0000}'), ccc::NotReordered);
    }

    #[test]
    fn test_number() {
        assert_eq!(CCC::of('\u{0000}').number(), 0);
        assert_eq!(CCC::of('\u{0300}').number(), 230);
        assert_eq!(CCC::of('\u{0315}').number(), 232);
        assert_eq!(CCC::of('\u{1e94a}').number(), 7);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", CCC::of('\u{0000}')), "0");
        assert_eq!(format!("{}", CCC::of('\u{0300}')), "230");
    }
}
