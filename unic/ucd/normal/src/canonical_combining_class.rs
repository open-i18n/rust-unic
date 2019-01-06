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

//! Accessor for `Canonical_Combining_Class` (ccc) property
//!
//! Reference: <http://unicode.org/reports/tr44/#Canonical_Combining_Class_Values>

use core::fmt;

use unic_char_property::{CharProperty, NumericCharProperty, TotalCharProperty};

/// Represents `Canonical_Combining_Class` property of a Unicode character.
///
/// * <http://unicode.org/reports/tr44/#Canonical_Combining_Class>
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanonicalCombiningClass(u8);

impl CharProperty for CanonicalCombiningClass {
    fn prop_abbr_name() -> &'static str {
        "ccc"
    }

    fn prop_long_name() -> &'static str {
        "Canonical_Combining_Class"
    }

    fn prop_human_name() -> &'static str {
        "Canonical Combining Class"
    }
}

impl TotalCharProperty for CanonicalCombiningClass {
    fn of(ch: char) -> Self {
        Self::of(ch)
    }
}

impl NumericCharProperty<u8> for CanonicalCombiningClass {
    /// Get numeric value for character property value
    fn number(&self) -> u8 {
        Self::number(self)
    }
}

mod data {
    use super::CanonicalCombiningClass;
    use unic_char_property::tables::CharDataTable;
    pub const CANONICAL_COMBINING_CLASS_VALUES: CharDataTable<CanonicalCombiningClass> =
        include!("../tables/canonical_combining_class_values.rsv");
}

#[cfg_attr(rustfmt, rustfmt_skip)]  // We want the consts ordered by value.
#[allow(non_upper_case_globals)]
impl CanonicalCombiningClass {
    /// Find the character `Canonical_Combining_Class` property value.
    pub fn of(ch: char) -> CanonicalCombiningClass {
        data::CANONICAL_COMBINING_CLASS_VALUES.find_or_default(ch)
    }

    // == Named values ==
    // TODO(DOCS): Add reference.
    /// Spacing and enclosing marks; also many vowel and consonant signs, even if nonspacing
    pub const NotReordered: CanonicalCombiningClass = CanonicalCombiningClass(0);
    /// Marks which overlay a base letter or symbol
    pub const Overlay: CanonicalCombiningClass = CanonicalCombiningClass(1);
    /// Diacritic nukta marks in Brahmi-derived scripts
    pub const Nukta: CanonicalCombiningClass = CanonicalCombiningClass(7);
    /// Hiragana/Katakana voicing marks
    pub const KanaVoicing: CanonicalCombiningClass = CanonicalCombiningClass(8);
    /// Viramas
    pub const Virama: CanonicalCombiningClass = CanonicalCombiningClass(9);
    /// Marks attached at the bottom left
    pub const AttatchedBelowLeft: CanonicalCombiningClass = CanonicalCombiningClass(200);
    /// Marks attached directly below
    pub const AttachedBelow: CanonicalCombiningClass = CanonicalCombiningClass(202);
    /// Marks attached at the bottom right
    pub const AttachedBelowRight: CanonicalCombiningClass = CanonicalCombiningClass(204);
    /// Marks attached to the left
    pub const AttachedLeft: CanonicalCombiningClass = CanonicalCombiningClass(208);
    /// Marks attached to the right
    pub const AttachedRight: CanonicalCombiningClass = CanonicalCombiningClass(210);
    /// Marks attached at the top left
    pub const AttachedAboveLeft: CanonicalCombiningClass = CanonicalCombiningClass(212);
    /// Marks attached directly above
    pub const AttatchedAbove: CanonicalCombiningClass = CanonicalCombiningClass(214);
    /// Marks attached at the top right
    pub const AttatchedAboveRight: CanonicalCombiningClass = CanonicalCombiningClass(216);
    /// Distinct marks at the bottom left
    pub const BelowLeft: CanonicalCombiningClass = CanonicalCombiningClass(218);
    /// Distinct marks directly below
    pub const Below: CanonicalCombiningClass = CanonicalCombiningClass(220);
    /// Distinct marks at the bottom right
    pub const BelowRight: CanonicalCombiningClass = CanonicalCombiningClass(222);
    /// Distinct marks to the left
    pub const Left: CanonicalCombiningClass = CanonicalCombiningClass(224);
    /// Distinct marks to the right
    pub const Right: CanonicalCombiningClass = CanonicalCombiningClass(226);
    /// Distinct marks at the top left
    pub const AboveLeft: CanonicalCombiningClass = CanonicalCombiningClass(228);
    /// Distinct marks directly above
    pub const Above: CanonicalCombiningClass = CanonicalCombiningClass(230);
    /// Distinct marks at the top right
    pub const AboveRight: CanonicalCombiningClass = CanonicalCombiningClass(232);
    /// Distinct marks subtending two bases
    pub const DoubleBelow: CanonicalCombiningClass = CanonicalCombiningClass(233);
    /// Distinct marks extending above two bases
    pub const DoubleAbove: CanonicalCombiningClass = CanonicalCombiningClass(234);
    /// Greek iota subscript only
    pub const IotaSubscript: CanonicalCombiningClass = CanonicalCombiningClass(240);
}

impl fmt::Display for CanonicalCombiningClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.number())
    }
}

impl CanonicalCombiningClass {
    /// Get numeric `Canonical_Combining_Class` value
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

    #[test]
    fn test_ascii() {
        assert_eq!(CCC::of('\u{0000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{0040}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{0041}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{0062}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{007F}'), CCC::NotReordered);
    }

    #[test]
    fn test_bmp() {
        // Combining Diacritical Marks
        assert_eq!(CCC::of('\u{0300}'), CCC::Above);
        assert_eq!(CCC::of('\u{0314}'), CCC::Above);
        assert_eq!(CCC::of('\u{0315}'), CCC::AboveRight);
        assert_eq!(CCC::of('\u{0316}'), CCC::Below);
        assert_eq!(CCC::of('\u{0319}'), CCC::Below);

        // Hebrew
        assert_eq!(CCC::of('\u{0590}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{05D0}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{05D1}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{05FF}'), CCC::NotReordered);

        // Arabic
        assert_eq!(CCC::of('\u{0600}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{0627}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{064B}'), CCC(27));
        assert_eq!(CCC::of('\u{064C}'), CCC(28));
        assert_eq!(CCC::of('\u{064D}'), CCC(29));
        assert_eq!(CCC::of('\u{064E}'), CCC(30));
        assert_eq!(CCC::of('\u{064F}'), CCC(31));
        assert_eq!(CCC::of('\u{0650}'), CCC(32));
        assert_eq!(CCC::of('\u{0651}'), CCC(33));
        assert_eq!(CCC::of('\u{0652}'), CCC(34));

        assert_eq!(CCC::of('\u{07BF}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{07C0}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{085F}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{0860}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{0870}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{089F}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{08A0}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{089F}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{08FF}'), CCC::Above);

        //  Currency Symbols
        assert_eq!(CCC::of('\u{20A0}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{20CF}'), CCC::NotReordered);

        // Arabic Presentation Forms
        assert_eq!(CCC::of('\u{FB1D}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FB4F}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FB50}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FDCF}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FDF0}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FDFF}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FE70}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FEFE}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FEFF}'), CCC::NotReordered);

        // noncharacters
        assert_eq!(CCC::of('\u{FDD0}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FDD1}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FDEE}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FDEF}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FFFE}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{FFFF}'), CCC::NotReordered);
    }

    #[test]
    fn test_smp() {
        assert_eq!(CCC::of('\u{10000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{101fc}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{101fd}'), CCC::Below);
        assert_eq!(CCC::of('\u{101fe}'), CCC::NotReordered);

        assert_eq!(CCC::of('\u{1e000}'), CCC::Above);

        assert_eq!(CCC::of('\u{1e949}'), CCC::Above);
        assert_eq!(CCC::of('\u{1e94a}'), CCC(7));
        assert_eq!(CCC::of('\u{1e94b}'), CCC::NotReordered);

        assert_eq!(CCC::of('\u{1efff}'), CCC::NotReordered);

        // noncharacters
        assert_eq!(CCC::of('\u{1fffe}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{1ffff}'), CCC::NotReordered);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(CCC::of('\u{30000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{40000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{50000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{60000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{70000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{80000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{90000}'), CCC::NotReordered);
        assert_eq!(CCC::of('\u{a0000}'), CCC::NotReordered);
    }

    #[test]
    fn test_number() {
        assert_eq!(CCC::of('\u{0000}').number(), 0);
        assert_eq!(CCC::of('\u{0300}').number(), 230);
        assert_eq!(CCC::of('\u{0315}').number(), 232);
        assert_eq!(CCC::of('\u{1e94a}').number(), 7);
    }
}
