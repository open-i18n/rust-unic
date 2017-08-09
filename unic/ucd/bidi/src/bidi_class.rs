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


use std::fmt;

use unic_utils::CharDataTable;
use unic_utils::char_property::{CompleteCharProperty, EnumeratedCharProperty};


/// Represents the Unicode character
/// [*Bidi_Class*](http://www.unicode.org/reports/tr44/#Bidi_Class) property, also known as the
/// *bidirectional character type*.
///
/// * <http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types>
/// * <http://www.unicode.org/reports/tr44/#Bidi_Class_Values>
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
#[allow(missing_docs)]
pub enum BidiClass {
    ArabicLetter,
    ArabicNumber,
    ParagraphSeparator,
    BoundaryNeutral,
    CommonSeparator,
    EuropeanNumber,
    EuropeanSeparator,
    EuropeanTerminator,
    FirstStrongIsolate,
    LeftToRight,
    LeftToRightEmbedding,
    LeftToRightIsolate,
    LeftToRightOverride,
    NonspacingMark,
    OtherNeutral,
    PopDirectionalFormat,
    PopDirectionalIsolate,
    RightToLeft,
    RightToLeftEmbedding,
    RightToLeftIsolate,
    RightToLeftOverride,
    SegmentSeparator,
    WhiteSpace,
    // [UNIC_UPDATE_ON_UNICODE_UPDATE] Source: `tables/bidi_class_type.rsv`
}

impl CompleteCharProperty for BidiClass {
    fn of(ch: char) -> Self {
        Self::of(ch)
    }
}

impl EnumeratedCharProperty<BidiClass> for BidiClass {
    fn all_values() -> &'static [Self] {
        Self::all_values()
    }
}


/// Abbreviated name aliases for
/// [*Bidi_Class*](http://www.unicode.org/reports/tr44/#Bidi_Class) property.
///
/// <http://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt#Bidi_Class>
pub mod abbr_names {
    pub use BidiClass::ArabicLetter as AL;
    pub use BidiClass::ArabicNumber as AN;
    pub use BidiClass::ParagraphSeparator as B;
    pub use BidiClass::BoundaryNeutral as BN;
    pub use BidiClass::CommonSeparator as CS;
    pub use BidiClass::EuropeanNumber as EN;
    pub use BidiClass::EuropeanSeparator as ES;
    pub use BidiClass::EuropeanTerminator as ET;
    pub use BidiClass::FirstStrongIsolate as FSI;
    pub use BidiClass::LeftToRight as L;
    pub use BidiClass::LeftToRightEmbedding as LRE;
    pub use BidiClass::LeftToRightIsolate as LRI;
    pub use BidiClass::LeftToRightOverride as LRO;
    pub use BidiClass::NonspacingMark as NSM;
    pub use BidiClass::OtherNeutral as ON;
    pub use BidiClass::PopDirectionalFormat as PDF;
    pub use BidiClass::PopDirectionalIsolate as PDI;
    pub use BidiClass::RightToLeft as R;
    pub use BidiClass::RightToLeftEmbedding as RLE;
    pub use BidiClass::RightToLeftIsolate as RLI;
    pub use BidiClass::RightToLeftOverride as RLO;
    pub use BidiClass::SegmentSeparator as S;
    pub use BidiClass::WhiteSpace as WS;
    // [UNIC_UPDATE_ON_UNICODE_UPDATE] Source: `tables/bidi_class_type.rsv`
}


use self::abbr_names::*;

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

impl BidiClass {
    /// Find the character *Bidi_Class* property value.
    pub fn of(ch: char) -> BidiClass {
        // UCD/extracted/DerivedBidiClass.txt: "All code points not explicitly listed
        // for Bidi_Class have the value Left_To_Right (L)."
        *BIDI_CLASS_TABLE.find_or(ch, &L)
    }

    /// Exhaustive list of all `BidiClass` property values.
    pub fn all_values() -> &'static [BidiClass] {
        use BidiClass::*;
        const ALL_VALUES: &[BidiClass] = &[
            ArabicLetter,
            ArabicNumber,
            ParagraphSeparator,
            BoundaryNeutral,
            CommonSeparator,
            EuropeanNumber,
            EuropeanSeparator,
            EuropeanTerminator,
            FirstStrongIsolate,
            LeftToRight,
            LeftToRightEmbedding,
            LeftToRightIsolate,
            LeftToRightOverride,
            NonspacingMark,
            OtherNeutral,
            PopDirectionalFormat,
            PopDirectionalIsolate,
            RightToLeft,
            RightToLeftEmbedding,
            RightToLeftIsolate,
            RightToLeftOverride,
            SegmentSeparator,
            WhiteSpace,
        ];
        ALL_VALUES
    }

    /// Abbreviated name of the *Bidi_Class* property value.
    ///
    /// <http://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt#Bidi_Class>
    pub fn abbr_name(&self) -> &str {
        match *self {
            BidiClass::ArabicLetter => "AL",
            BidiClass::ArabicNumber => "AN",
            BidiClass::ParagraphSeparator => "B",
            BidiClass::BoundaryNeutral => "BN",
            BidiClass::CommonSeparator => "CS",
            BidiClass::EuropeanNumber => "EN",
            BidiClass::EuropeanSeparator => "ES",
            BidiClass::EuropeanTerminator => "ET",
            BidiClass::FirstStrongIsolate => "FSI",
            BidiClass::LeftToRight => "L",
            BidiClass::LeftToRightEmbedding => "LRE",
            BidiClass::LeftToRightIsolate => "LRI",
            BidiClass::LeftToRightOverride => "LRO",
            BidiClass::NonspacingMark => "NSM",
            BidiClass::OtherNeutral => "ON",
            BidiClass::PopDirectionalFormat => "PDF",
            BidiClass::PopDirectionalIsolate => "PDI",
            BidiClass::RightToLeft => "R",
            BidiClass::RightToLeftEmbedding => "RLE",
            BidiClass::RightToLeftIsolate => "RLI",
            BidiClass::RightToLeftOverride => "RLO",
            BidiClass::SegmentSeparator => "S",
            BidiClass::WhiteSpace => "WS",
        }
    }

    /// Human-readable description of the *Bidi_Class* property value.
    ///
    /// <http://www.unicode.org/reports/tr9/#Table_Bidirectional_Character_Types>
    #[inline]
    pub fn display(&self) -> &str {
        match *self {
            // Strong
            L => "Left-to-Right",
            R => "Right-to-Left",
            AL => "Right-to-Left Arabic",

            // Weak
            EN => "European Number",
            ES => "European Number Separator",
            ET => "European Number Terminator",
            AN => "Arabic Number",
            CS => "Common Number Separator",
            NSM => "Nonspacing Mark",
            BN => "Boundary Neutral",

            // Neutral
            B => "Paragraph Separator",
            S => "Segment Separator",
            WS => "Whitespace",
            ON => "Other Neutrals",

            // Explicit Formatting
            LRE => "Left-to-Right Embedding",
            LRO => "Left-to-Right Override",
            RLE => "Right-to-Left Embedding",
            RLO => "Right-to-Left Override",
            PDF => "Pop Directional Format",
            LRI => "Left-to-Right Isolate",
            RLI => "Right-to-Left Isolate",
            FSI => "First Strong Isolate",
            PDI => "Pop Directional Isolate",
        }
    }

    /// If the `BidiClass` has strong or explicit Left-to-Right direction.
    #[inline]
    pub fn category(&self) -> BidiClassCategory {
        match *self {
            L | R | AL => BidiClassCategory::Strong,
            EN | ES | ET | AN | CS | NSM | BN => BidiClassCategory::Weak,
            B | S | WS | ON => BidiClassCategory::Neutral,
            LRE | LRO | RLE | RLO | PDF | LRI | RLI | FSI | PDI => {
                BidiClassCategory::ExplicitFormatting
            }
        }
    }

    /// If the `BidiClass` has strong or explicit Left-to-Right direction.
    #[inline]
    pub fn is_ltr(&self) -> bool {
        match *self {
            L | LRE | LRO | LRI => true,
            _ => false,
        }
    }

    /// If the `BidiClass` has strong or explicit Right-To-Left direction.
    #[inline]
    pub fn is_rtl(&self) -> bool {
        match *self {
            AL | R | RLE | RLO | RLI => true,
            _ => false,
        }
    }
}


impl Default for BidiClass {
    fn default() -> Self {
        BidiClass::LeftToRight
    }
}


impl fmt::Display for BidiClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}


#[cfg(test)]
mod tests {
    use super::BidiClass;
    use super::abbr_names::*;

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
    fn test_display() {
        assert_eq!(format!("{}", L), "Left-to-Right");
        assert_eq!(format!("{}", R), "Right-to-Left");
        assert_eq!(format!("{}", AL), "Right-to-Left Arabic");
        assert_eq!(format!("{}", FSI), "First Strong Isolate");
    }
}
