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

//! Accessor for `Decomposition_Type` (dt) property

use unic_char_property::PartialCharProperty;
use unic_ucd_hangul::is_syllable;

use crate::composition::{canonical_decomposition, data};

char_property! {
    /// Represents the Unicode character
    /// [`Decomposition_Type`](https://www.unicode.org/reports/tr44/#Decomposition_Type) property.
    ///
    /// * <https://www.unicode.org/reports/tr44/#Character_Decomposition_Mappings>
    pub enum DecompositionType {
        abbr => "dt";
        long => "Decomposition_Type";
        human => "Decomposition Type";

        #[allow(missing_docs)]
        Canonical {
            abbr => Can,
            long => Canonical,
            human => "Canonical",
        }

        /// Otherwise unspecified compatibility character
        Compat {
            abbr => Com,
            long => Compat,
            human => "Unspecified",
        }

        /// Encircled form
        Circle {
            abbr => Enc,
            long => Circle,
            human => "Encircled",
        }

        /// Final presentation form (Arabic)
        Final {
            abbr => Fin,
            long => Final,
            human => "Arabic Final",
        }

        /// Font variant (for example, a blackletter form)
        Font {
            abbr => Font,
            long => Font,
            human => "Font Variant",
        }

        /// Vulgar fraction form
        Fraction {
            abbr => Fra,
            long => Fraction,
            human => "Vulgar Fraction",
        }

        /// Initial presentation form (Arabic)
        Initial {
            abbr => Init,
            long => Initial,
            human => "Arabic Initial",
        }

        /// Isolated presentation form (Arabic)
        Isolated {
            abbr => Iso,
            long => Isolated,
            human => "Arabic Isolated",
        }

        /// Medial presentation form (Arabic)
        Medial {
            abbr => Med,
            long => Medial,
            human => "Arabic Medial",
        }

        /// Narrow (or hankaku) compatibility character
        Narrow {
            abbr => Nar,
            long => Narrow,
            human => "Narrow",
        }

        /// No-break version of a space or hyphen
        NoBreak {
            abbr => Nb,
            long => Nobreak,
            human => "No-Break",
        }

        /// Small variant form (CNS compatibility)
        Small {
            abbr => Sml,
            long => Small,
            human => "Small",
        }

        /// CJK squared font variant
        Square {
            abbr => Sqr,
            long => Square,
            human => "CJK Squared",
        }

        /// Subscript form
        Sub {
            abbr => Sub,
            long => Sub,
            human => "Subscript",
        }

        /// Superscript form
        Super {
            abbr => Sup,
            long => Super,
            human => "Superscript",
        }

        /// Vertical layout presentation form
        Vertical {
            abbr => Vert,
            long => Vertical,
            human => "Vertical Layout",
        }

        /// Wide (or zenkaku) compatibility character
        Wide {
            abbr => Wide,
            long => Wide,
            human => "Wide",
        }
    }

    #[allow(missing_docs)]
    pub mod abbr_names for abbr;

    #[allow(missing_docs)]
    pub mod long_names for long;
}

impl PartialCharProperty for DecompositionType {
    fn of(ch: char) -> Option<Self> {
        Self::of(ch)
    }
}

impl DecompositionType {
    /// Find the DecompositionType of the character.
    pub fn of(ch: char) -> Option<DecompositionType> {
        // First, check for Hangul Syllables and other canonical decompositions
        if is_syllable(ch) || canonical_decomposition(ch).is_some() {
            return Some(DecompositionType::Canonical);
        }
        data::COMPATIBILITY_DECOMPOSITION_MAPPING
            .find(ch)
            .map(|it| it.0)
    }
}

#[cfg(test)]
mod tests {
    use super::DecompositionType as DT;
    use unic_char_property::EnumeratedCharProperty;

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
        assert_eq!(DT::of('\u{a0}'), Some(DT::NoBreak));
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
    fn test_abbr_name() {
        assert_eq!(DT::Canonical.abbr_name(), "Can");
        assert_eq!(DT::NoBreak.abbr_name(), "Nb");
    }

    #[test]
    fn test_long_name() {
        assert_eq!(DT::Canonical.long_name(), "Canonical");
        assert_eq!(DT::NoBreak.long_name(), "Nobreak");
    }

    #[test]
    fn test_human_name() {
        assert_eq!(DT::Canonical.human_name(), "Canonical");
        assert_eq!(DT::NoBreak.human_name(), "No-Break");
    }
}
