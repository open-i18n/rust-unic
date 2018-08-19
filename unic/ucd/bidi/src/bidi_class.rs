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

//! Unicode `Bidi_Class` Character Property.

use unic_char_property::TotalCharProperty;

char_property! {
    /// Represents the Unicode character
    /// [`Bidi_Class`](https://www.unicode.org/reports/tr44/#Bidi_Class) property, also known as the
    /// *bidirectional character type*.
    ///
    /// * <https://www.unicode.org/reports/tr9/#Bidirectional_Character_Types>
    /// * <https://www.unicode.org/reports/tr44/#Bidi_Class_Values>
    pub enum BidiClass {
        abbr => "bc";
        long => "Bidi_Class";
        human => "Bidirectional Class";

        /// A strong Right-to-Left (Arabic-type) character
        ArabicLetter {
            abbr => AL,
            long => Arabic_Letter,
            human => "Right-to-Left Arabic",
        }

        /// A (non-Eastern) Arabic-Indic digit
        ArabicNumber {
            abbr => AN,
            long => Arabic_Number,
            human => "Arabic Number",
        }

        /// A newline character
        ParagraphSeparator {
            abbr => B,
            long => Paragraph_Separator,
            human => "Paragraph Separator",
        }

        /// Most format characters, control codes, and noncharacters
        BoundaryNeutral {
            abbr => BN,
            long => Boundary_Neutral,
            human => "Neutral Boundary",
        }

        /// A comma, colon, or slash
        CommonSeparator {
            abbr => CS,
            long => Common_Separator,
            human => "Common Number Separator",
        }

        /// A ASCII digit or Eastern Arabic-Indic digit
        EuropeanNumber {
            abbr => EN,
            long => European_Number,
            human => "European Number",
        }

        /// A plus or minus sign
        EuropeanSeparator {
            abbr => ES,
            long => European_Separator,
            human => "European Number Separator",
        }

        /// A terminator in a numeric format context (including currency signs)
        EuropeanTerminator {
            abbr => ET,
            long => European_Terminator,
            human => "European Number Terminator",
        }

        /// U+2068: The first strong isolate control
        FirstStrongIsolate {
            abbr => FSI,
            long => First_Strong_Isolate,
            human => "First Strong Isolate",
        }

        /// A strong Left-to-Right character
        LeftToRight {
            abbr => L,
            long => Left_To_Right,
            human => "Left-to-Right",
        }

        /// U+202A: the Left-to-Right embedding control
        LeftToRightEmbedding {
            abbr => LRE,
            long => Left_To_Right_Embedding,
            human => "Left-to-Right Embedding",
        }

        /// U+2066: the Left-to-Right isolate control
        LeftToRightIsolate {
            abbr => LRI,
            long => Left_To_Right_Isolate,
            human => "Left-to-Right Isolate",
        }

        /// U+202D: the Left-to-Right override control
        LeftToRightOverride {
            abbr => LRO,
            long => Left_To_Right_Override,
            human => "LeftToRightOverride",
        }

        /// A nonspacing mark
        NonspacingMark {
            abbr => NSM,
            long => Nonspacing_Mark,
            human => "Nonspacing Mark",
        }

        /// Symbols and Punctuation not in a different category
        OtherNeutral {
            abbr => ON,
            long => Other_Neutral,
            human => "OtherNeutral",
        }

        /// U+202C: terminates an embedding or override control
        PopDirectionalFormat {
            abbr => PDF,
            long => Pop_Directional_Format,
            human => "Pop Directional Format",
        }

        /// U+2069: terminates an isolate control
        PopDirectionalIsolate {
            abbr => PDI,
            long => Pop_Directional_Isolate,
            human => "PopDirectionalIsolate",
        }

        /// A strong Right-to-Left (non-Arabic-type) character
        RightToLeft {
            abbr => R,
            long => Right_To_Left,
            human => "Right-to-Left",
        }

        /// U+202B: The Right-to-Left embedding control
        RightToLeftEmbedding {
            abbr => RLE,
            long => Right_To_Left_Embedding,
            human => "Right-to-Left Embedding",
        }

        /// U+2067: The Right-to-Left isolate control
        RightToLeftIsolate {
            abbr => RLI,
            long => Right_To_Left_Isolate,
            human => "Right-to-Left Isolate",
        }

        /// U+202E: The Right-to-Left override control
        RightToLeftOverride {
            abbr => RLO,
            long => Right_To_Left_Override,
            human => "Right-to-Left Override",
        }

        /// A segment-related control code
        SegmentSeparator {
            abbr => S,
            long => Segment_Separator,
            human => "Segment Separator",
        }

        /// Whitespace
        WhiteSpace {
            abbr => WS,
            long => White_Space,
            human => "Whitespace",
        }
    }

    /// Abbreviated name aliases for the
    /// [`Bidi_Class`](https://www.unicode.org/reports/tr44/#Bidi_Class) property.
    ///
    /// ## See Also
    ///
    /// - <https://www.unicode.org/reports/tr44/#Bidi_Class_Values>
    /// - <https://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt#Bidi_Class>
    pub mod abbr_names for abbr;

    /// Long name aliases for the
    /// [`Bidi_Class`](https://www.unicode.org/reports/tr44/#Bidi_Class) property.
    ///
    /// ## See Also
    ///
    /// - <https://www.unicode.org/reports/tr44/#Bidi_Class_Values>
    /// - <https://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt#Bidi_Class>
    pub mod long_names for long;
}

impl TotalCharProperty for BidiClass {
    fn of(ch: char) -> Self {
        Self::of(ch)
    }
}

/// UCD/extracted/DerivedBidiClass.txt:
/// "All code points not explicitly listed for `Bidi_Class` have the value `Left_To_Right` (`L`)."
impl Default for BidiClass {
    #[inline]
    fn default() -> Self {
        BidiClass::LeftToRight
    }
}

mod data {
    use super::abbr_names::*;
    use unic_char_property::tables::CharDataTable;
    pub const BIDI_CLASS_TABLE: CharDataTable<super::BidiClass> =
        include!("../tables/bidi_class.rsv");
}

impl BidiClass {
    /// Find the character `Bidi_Class` property value.
    pub fn of(ch: char) -> BidiClass {
        data::BIDI_CLASS_TABLE.find_or_default(ch)
    }

    /// If the `BidiClass` has strong or explicit Left-to-Right direction.
    #[inline]
    pub fn category(&self) -> BidiClassCategory {
        match *self {
            BidiClass::LeftToRight | BidiClass::RightToLeft | BidiClass::ArabicLetter => {
                BidiClassCategory::Strong
            }

            BidiClass::EuropeanNumber
            | BidiClass::EuropeanSeparator
            | BidiClass::EuropeanTerminator
            | BidiClass::ArabicNumber
            | BidiClass::CommonSeparator
            | BidiClass::NonspacingMark
            | BidiClass::BoundaryNeutral => BidiClassCategory::Weak,

            BidiClass::ParagraphSeparator
            | BidiClass::SegmentSeparator
            | BidiClass::WhiteSpace
            | BidiClass::OtherNeutral => BidiClassCategory::Neutral,

            BidiClass::LeftToRightEmbedding
            | BidiClass::LeftToRightOverride
            | BidiClass::RightToLeftEmbedding
            | BidiClass::RightToLeftOverride
            | BidiClass::PopDirectionalFormat
            | BidiClass::LeftToRightIsolate
            | BidiClass::RightToLeftIsolate
            | BidiClass::FirstStrongIsolate
            | BidiClass::PopDirectionalIsolate => BidiClassCategory::ExplicitFormatting,
        }
    }

    /// If the `BidiClass` has strong or explicit Left-to-Right direction.
    #[inline]
    pub fn is_ltr(&self) -> bool {
        match *self {
            BidiClass::LeftToRight
            | BidiClass::LeftToRightEmbedding
            | BidiClass::LeftToRightOverride
            | BidiClass::LeftToRightIsolate => true,
            _ => false,
        }
    }

    /// If the `BidiClass` has strong or explicit Right-To-Left direction.
    #[inline]
    pub fn is_rtl(&self) -> bool {
        match *self {
            BidiClass::RightToLeft
            | BidiClass::ArabicLetter
            | BidiClass::RightToLeftEmbedding
            | BidiClass::RightToLeftOverride
            | BidiClass::RightToLeftIsolate => true,
            _ => false,
        }
    }
}

/// Represents **Category** of Unicode character `Bidi_Class` property, as demostrated under
/// "Table 4. Bidirectional Character Types".
///
/// * <https://www.unicode.org/reports/tr9/#Table_Bidirectional_Character_Types>
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

/// Methods for `Bidi_Class` character property.
pub trait CharBidiClass {
    /// Get `BidiClass` of the character.
    fn bidi_class(self) -> BidiClass;

    /// Whether the character has *left-to-right* (LTR) bidi directionality.
    fn is_ltr(self) -> bool;

    /// Whether the character has *right-to-left* (RTL) bidi directionality.
    fn is_rtl(self) -> bool;
}

impl CharBidiClass for char {
    #[inline]
    fn bidi_class(self) -> BidiClass {
        BidiClass::of(self)
    }

    #[inline]
    fn is_ltr(self) -> bool {
        BidiClass::of(self).is_ltr()
    }

    #[inline]
    fn is_rtl(self) -> bool {
        BidiClass::of(self).is_rtl()
    }
}

/// Methods for `Bidi_Class` character properties of string types.
pub trait StrBidiClass {
    /// Whether the string has any *explicit* bidi formatting character.
    fn has_bidi_explicit(&self) -> bool;

    /// Whether the string has any character with *left-to-right* (LTR) bidi directionality.
    fn has_ltr(&self) -> bool;

    /// Whether the string has any character with *right-to-left* (RTL) bidi directionality.
    fn has_rtl(&self) -> bool;
}

impl StrBidiClass for str {
    #[inline]
    fn has_bidi_explicit(&self) -> bool {
        self.chars()
            .any(|ch| ch.bidi_class().category() == BidiClassCategory::ExplicitFormatting)
    }

    #[inline]
    fn has_ltr(&self) -> bool {
        self.chars().any(|ch| ch.is_rtl())
    }

    #[inline]
    fn has_rtl(&self) -> bool {
        self.chars().any(|ch| ch.is_rtl())
    }
}

#[cfg(test)]
mod tests {
    use super::abbr_names::*;
    use super::BidiClass;
    use unic_char_property::EnumeratedCharProperty;

    #[test]
    fn test_ascii() {
        assert_eq!(BidiClass::of('\u{0000}'), BN);
        assert_eq!(BidiClass::of('\u{0040}'), ON);
        assert_eq!(BidiClass::of('\u{0041}'), L);
        assert_eq!(BidiClass::of('\u{0062}'), L);
        assert_eq!(BidiClass::of('\u{007F}'), BN);
    }

    #[test]
    fn test_bmp() {
        // Hebrew
        assert_eq!(BidiClass::of('\u{0590}'), R);
        assert_eq!(BidiClass::of('\u{05D0}'), R);
        assert_eq!(BidiClass::of('\u{05D1}'), R);
        assert_eq!(BidiClass::of('\u{05FF}'), R);

        // Arabic
        assert_eq!(BidiClass::of('\u{0600}'), AN);
        assert_eq!(BidiClass::of('\u{0627}'), AL);
        assert_eq!(BidiClass::of('\u{07BF}'), AL);

        // Default R + Arabic Extras
        assert_eq!(BidiClass::of('\u{07C0}'), R);
        assert_eq!(BidiClass::of('\u{085F}'), R);
        assert_eq!(BidiClass::of('\u{0860}'), AL);
        assert_eq!(BidiClass::of('\u{0870}'), R);
        assert_eq!(BidiClass::of('\u{089F}'), R);
        assert_eq!(BidiClass::of('\u{08A0}'), AL);
        assert_eq!(BidiClass::of('\u{089F}'), R);
        assert_eq!(BidiClass::of('\u{08FF}'), NSM);

        // Default ET
        assert_eq!(BidiClass::of('\u{20A0}'), ET);
        assert_eq!(BidiClass::of('\u{20CF}'), ET);

        // Arabic Presentation Forms
        assert_eq!(BidiClass::of('\u{FB1D}'), R);
        assert_eq!(BidiClass::of('\u{FB4F}'), R);
        assert_eq!(BidiClass::of('\u{FB50}'), AL);
        assert_eq!(BidiClass::of('\u{FDCF}'), AL);
        assert_eq!(BidiClass::of('\u{FDF0}'), AL);
        assert_eq!(BidiClass::of('\u{FDFF}'), AL);
        assert_eq!(BidiClass::of('\u{FE70}'), AL);
        assert_eq!(BidiClass::of('\u{FEFE}'), AL);
        assert_eq!(BidiClass::of('\u{FEFF}'), BN);

        // noncharacters
        assert_eq!(BidiClass::of('\u{FDD0}'), L);
        assert_eq!(BidiClass::of('\u{FDD1}'), L);
        assert_eq!(BidiClass::of('\u{FDEE}'), L);
        assert_eq!(BidiClass::of('\u{FDEF}'), L);
        assert_eq!(BidiClass::of('\u{FFFE}'), L);
        assert_eq!(BidiClass::of('\u{FFFF}'), L);
    }

    #[test]
    fn test_smp() {
        // Default AL + R
        assert_eq!(BidiClass::of('\u{10800}'), R);
        assert_eq!(BidiClass::of('\u{10FFF}'), R);
        assert_eq!(BidiClass::of('\u{1E800}'), R);
        assert_eq!(BidiClass::of('\u{1EDFF}'), R);
        assert_eq!(BidiClass::of('\u{1EE00}'), AL);
        assert_eq!(BidiClass::of('\u{1EEFF}'), AL);
        assert_eq!(BidiClass::of('\u{1EF00}'), R);
        assert_eq!(BidiClass::of('\u{1EFFF}'), R);
    }

    #[test]
    fn test_unassigned_planes() {
        assert_eq!(BidiClass::of('\u{30000}'), L);
        assert_eq!(BidiClass::of('\u{40000}'), L);
        assert_eq!(BidiClass::of('\u{50000}'), L);
        assert_eq!(BidiClass::of('\u{60000}'), L);
        assert_eq!(BidiClass::of('\u{70000}'), L);
        assert_eq!(BidiClass::of('\u{80000}'), L);
        assert_eq!(BidiClass::of('\u{90000}'), L);
        assert_eq!(BidiClass::of('\u{a0000}'), L);
    }

    #[test]
    fn test_abbr_name() {
        assert_eq!(AL.abbr_name(), "AL");
        assert_eq!(FSI.abbr_name(), "FSI");
    }

    #[test]
    fn test_long_name() {
        assert_eq!(AL.long_name(), "Arabic_Letter");
        assert_eq!(FSI.long_name(), "First_Strong_Isolate");
    }

    #[test]
    fn test_human_name() {
        assert_eq!(AL.human_name(), "Right-to-Left Arabic");
        assert_eq!(FSI.human_name(), "First Strong Isolate");
    }

    #[test]
    fn test_char_trait() {
        use super::{BidiClass, BidiClassCategory, CharBidiClass};

        let ch = '\u{0028}'; // U+0028 LEFT PARENTHESIS "("
        assert_eq!(ch.bidi_class(), BidiClass::OtherNeutral);
        assert_eq!(ch.bidi_class().category(), BidiClassCategory::Neutral);
        assert!(!ch.is_ltr());
        assert!(!ch.is_rtl());

        let ch = '\u{0041}'; // U+0041 LATIN CAPITAL LETTER A "A"
        assert_eq!(ch.bidi_class(), BidiClass::LeftToRight);
        assert_eq!(ch.bidi_class().category(), BidiClassCategory::Strong);
        assert!(ch.is_ltr());
        assert!(!ch.is_rtl());

        let ch = '\u{05D0}'; // U+05D0 HEBREW LETTER ALEF "א"
        assert_eq!(ch.bidi_class(), BidiClass::RightToLeft);
        assert_eq!(ch.bidi_class().category(), BidiClassCategory::Strong);
        assert!(!ch.is_ltr());
        assert!(ch.is_rtl());

        let ch = '\u{0627}'; // U+0627 ARABIC LETTER ALEF "ا"
        assert_eq!(ch.bidi_class(), BidiClass::ArabicLetter);
        assert_eq!(ch.bidi_class().category(), BidiClassCategory::Strong);
        assert!(!ch.is_ltr());
        assert!(ch.is_rtl());
    }

    #[test]
    fn test_str_trait() {
        use super::StrBidiClass;

        let text = "";
        assert!(!text.has_bidi_explicit());
        assert!(!text.has_ltr());
        assert!(!text.has_rtl());

        let text = "[\u{0041}\u{05D0}\u{0627}\u{200e}]";
        assert!(!text.has_bidi_explicit());
        assert!(text.has_ltr());
        assert!(text.has_rtl());
    }
}
