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


//! Accessor for *Decomposition_Type* (dt) property

use std::fmt;

use unic_utils::CharDataTable;
use unic_char_property::{CharProperty, EnumeratedCharProperty, PartialCharProperty};

use composition::{canonical_decomposition, COMPATIBILITY_DECOMPOSITION_MAPPING};
use hangul;


/// Represents the Unicode character
/// [*Decomposition_Type*](http://www.unicode.org/reports/tr44/#Decomposition_Type) property.
///
/// * <http://www.unicode.org/reports/tr44/#Character_Decomposition_Mappings>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(missing_docs)]
pub enum DecompositionType {
    Canonical,
    Compat,
    Circle,
    Final,
    Font,
    Fraction,
    Initial,
    Isolated,
    Medial,
    Narrow,
    Nobreak,
    None,
    Small,
    Square,
    Sub,
    Super,
    Vertical,
    Wide,
}


impl CharProperty for DecompositionType {
    fn prop_abbr_name() -> &'static str {
        "dt"
    }

    fn prop_long_name() -> &'static str {
        "Decomposition_Type"
    }

    fn prop_human_name() -> &'static str {
        "Decomposition Type"
    }
}


impl PartialCharProperty for DecompositionType {
    fn of(ch: char) -> Option<Self> {
        Self::of(ch)
    }
}


impl EnumeratedCharProperty for DecompositionType {
    fn all_values() -> &'static [Self] {
        Self::all_values()
    }

    fn abbr_name(&self) -> &'static str {
        self.abbr_name()
    }
}


pub mod abbr_names {
    pub use DecompositionType::Canonical as Can;
    pub use DecompositionType::Compat as Com;
    pub use DecompositionType::Circle as Enc;
    pub use DecompositionType::Final as Fin;
    pub use DecompositionType::Font;
    pub use DecompositionType::Fraction as Fra;
    pub use DecompositionType::Initial as Init;
    pub use DecompositionType::Isolated as Iso;
    pub use DecompositionType::Medial as Med;
    pub use DecompositionType::Narrow as Nar;
    pub use DecompositionType::Nobreak as Nb;
    pub use DecompositionType::None;
    pub use DecompositionType::Small as Sml;
    pub use DecompositionType::Square as Sqr;
    pub use DecompositionType::Sub;
    pub use DecompositionType::Super as Sup;
    pub use DecompositionType::Vertical as Vert;
    pub use DecompositionType::Wide;
}


use self::DecompositionType::*;


impl DecompositionType {
    /// Find the DecompositionType of a single char.
    pub fn of(ch: char) -> Option<DecompositionType> {
        // First, check for Hangul Syllables and other canonical decompositions
        if hangul::is_syllable(ch) || canonical_decomposition(ch).is_some() {
            return Some(Canonical);
        }
        COMPATIBILITY_DECOMPOSITION_MAPPING.find(ch).map(|it| it.0)
    }

    /// Exhaustive list of all `DecompositionType` property values.
    pub fn all_values() -> &'static [DecompositionType] {
        use DecompositionType::*;
        const ALL_VALUES: &[DecompositionType] = &[
            Canonical,
            Compat,
            Circle,
            Final,
            Font,
            Fraction,
            Initial,
            Isolated,
            Medial,
            Narrow,
            Nobreak,
            None,
            Small,
            Square,
            Sub,
            Super,
            Vertical,
            Wide,
        ];
        ALL_VALUES
    }

    /// Abbreviated name of the *Decomposition_Type* property value.
    ///
    /// <http://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt#Decomposition_Type>
    pub fn abbr_name(&self) -> &'static str {
        use DecompositionType::*;
        match *self {
            Canonical => "Can",
            Compat => "Com",
            Circle => "Enc",
            Final => "Fin",
            Font => "Font",
            Fraction => "Fra",
            Initial => "Init",
            Isolated => "Iso",
            Medial => "Med",
            Narrow => "Nar",
            Nobreak => "Nb",
            None => "None",
            Small => "Sml",
            Square => "Sqr",
            Sub => "Sub",
            Super => "Sup",
            Vertical => "Vert",
            Wide => "Wide",
        }
    }

    /// Human-readable description of the property value.
    // TODO: Needs to be improved by returning long-name with underscores replaced by space.
    #[inline]
    pub fn display(&self) -> String {
        format!("{:?}", self).to_owned()
    }
}


impl fmt::Display for DecompositionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}


#[cfg(test)]
mod tests {
    use super::DecompositionType as DT;

    #[test]
    fn test_ascii() {
        assert_eq!(DT::of('\u{0000}'), None);
        assert_eq!(DT::of('\u{0040}'), None);
        assert_eq!(DT::of('\u{0041}'), None);
        assert_eq!(DT::of('\u{0062}'), None);
        assert_eq!(DT::of('\u{007F}'), None);
    }

    #[test]
    fn test_bmp() {
        // Compatibility
        assert_eq!(DT::of('\u{a0}'), Some(DT::Nobreak));
        assert_eq!(DT::of('\u{a8}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{aa}'), Some(DT::Super));
        assert_eq!(DT::of('\u{af}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b2}'), Some(DT::Super));
        assert_eq!(DT::of('\u{b3}'), Some(DT::Super));
        assert_eq!(DT::of('\u{b4}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b5}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b8}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{b9}'), Some(DT::Super));
        assert_eq!(DT::of('\u{ba}'), Some(DT::Super));
        assert_eq!(DT::of('\u{bc}'), Some(DT::Fraction));
        assert_eq!(DT::of('\u{bd}'), Some(DT::Fraction));
        assert_eq!(DT::of('\u{be}'), Some(DT::Fraction));

        // Canonical
        assert_eq!(DT::of('\u{c0}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{c1}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{d6}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{d9}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{10f}'), Some(DT::Canonical));

        // Combining Diacritical Marks
        assert_eq!(DT::of('\u{0300}'), None);
        assert_eq!(DT::of('\u{0314}'), None);
        assert_eq!(DT::of('\u{0315}'), None);
        assert_eq!(DT::of('\u{0316}'), None);
        assert_eq!(DT::of('\u{0319}'), None);

        // Hebrew
        assert_eq!(DT::of('\u{0590}'), None);
        assert_eq!(DT::of('\u{05D0}'), None);
        assert_eq!(DT::of('\u{05D1}'), None);
        assert_eq!(DT::of('\u{05FF}'), None);

        // Arabic
        assert_eq!(DT::of('\u{0600}'), None);
        assert_eq!(DT::of('\u{0627}'), None);
        assert_eq!(DT::of('\u{064B}'), None);
        assert_eq!(DT::of('\u{064C}'), None);
        assert_eq!(DT::of('\u{064D}'), None);
        assert_eq!(DT::of('\u{064E}'), None);
        assert_eq!(DT::of('\u{064F}'), None);
        assert_eq!(DT::of('\u{0650}'), None);
        assert_eq!(DT::of('\u{0651}'), None);
        assert_eq!(DT::of('\u{0652}'), None);

        assert_eq!(DT::of('\u{07BF}'), None);
        assert_eq!(DT::of('\u{07C0}'), None);
        assert_eq!(DT::of('\u{085F}'), None);
        assert_eq!(DT::of('\u{0860}'), None);
        assert_eq!(DT::of('\u{0870}'), None);
        assert_eq!(DT::of('\u{089F}'), None);
        assert_eq!(DT::of('\u{08A0}'), None);
        assert_eq!(DT::of('\u{089F}'), None);
        assert_eq!(DT::of('\u{08FF}'), None);

        //  Currency Symbols
        assert_eq!(DT::of('\u{20A0}'), None);
        assert_eq!(DT::of('\u{20CF}'), None);

        // Arabic Presentation Forms
        assert_eq!(DT::of('\u{FB1D}'), Some(DT::Canonical));
        assert_eq!(DT::of('\u{FB4F}'), Some(DT::Compat));
        assert_eq!(DT::of('\u{FB50}'), Some(DT::Isolated));
        assert_eq!(DT::of('\u{FDCF}'), None);
        assert_eq!(DT::of('\u{FDF0}'), Some(DT::Isolated));
        assert_eq!(DT::of('\u{FDFF}'), None);
        assert_eq!(DT::of('\u{FE70}'), Some(DT::Isolated));
        assert_eq!(DT::of('\u{FEFE}'), None);
        assert_eq!(DT::of('\u{FEFF}'), None);

        // noncharacters
        assert_eq!(DT::of('\u{FDD0}'), None);
        assert_eq!(DT::of('\u{FDD1}'), None);
        assert_eq!(DT::of('\u{FDEE}'), None);
        assert_eq!(DT::of('\u{FDEF}'), None);
        assert_eq!(DT::of('\u{FFFE}'), None);
        assert_eq!(DT::of('\u{FFFF}'), None);
    }

    #[test]
    fn test_smp() {
        assert_eq!(DT::of('\u{10000}'), None);
        assert_eq!(DT::of('\u{101fc}'), None);
        assert_eq!(DT::of('\u{101fd}'), None);
        assert_eq!(DT::of('\u{101fe}'), None);

        assert_eq!(DT::of('\u{1e000}'), None);

        assert_eq!(DT::of('\u{1e949}'), None);
        assert_eq!(DT::of('\u{1e94a}'), None);
        assert_eq!(DT::of('\u{1e94b}'), None);

        assert_eq!(DT::of('\u{1efff}'), None);

        // noncharacters
        assert_eq!(DT::of('\u{1fffe}'), None);
        assert_eq!(DT::of('\u{1ffff}'), None);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(DT::of('\u{30000}'), None);
        assert_eq!(DT::of('\u{40000}'), None);
        assert_eq!(DT::of('\u{50000}'), None);
        assert_eq!(DT::of('\u{60000}'), None);
        assert_eq!(DT::of('\u{70000}'), None);
        assert_eq!(DT::of('\u{80000}'), None);
        assert_eq!(DT::of('\u{90000}'), None);
        assert_eq!(DT::of('\u{a0000}'), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", DT::of('\u{a0}').unwrap()), "Nobreak");
    }

    #[test]
    fn test_abbr_name() {
        use super::abbr_names::*;
        assert_eq!(Can.abbr_name(), "Can");
    }
}
