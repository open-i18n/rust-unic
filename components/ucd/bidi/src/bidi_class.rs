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


char_property! {
    // TODO: Once 1.20 comes, add the rest of the enum variants' docs
    /// Represents the Unicode character
    /// [*Bidi_Class*](http://www.unicode.org/reports/tr44/#Bidi_Class) property, also known as the
    /// *bidirectional character type*.
    ///
    /// * <http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types>
    /// * <http://www.unicode.org/reports/tr44/#Bidi_Class_Values>
    pub enum BidiClass {
        // == Strong == //

        /// Any strong left-to-right character
        // ///
        // /// ***General Scope***
        // ///
        // /// LRM, most alphabetic, syllabic, Han ideographs,
        // /// non-European or non-Arabic digits, ...
        LeftToRight: L "Left-to-Right",

        /// Any strong right-to-left (non-Arabic-type) character
        // ///
        // /// ***General Scope***
        // ///
        // /// RLM, Hebrew alphabet, and related punctuation
        RightToLeft: R "Right-to-Left",

        /// Any strong right-to-left (Arabic-type) character
        // ///
        // /// ***General Scope***
        // ///
        // /// ALM, Arabic, Thaana, and Syriac alphabets,
        // /// most punctuation specific to those scripts, ...
        ArabicLetter: AL "Right-to-Left Arabic",

        // == Weak == //

        /// Any ASCII digit or Eastern Arabic-Indic digit
        // ///
        // /// ***General Scope***
        // ///
        // /// European digits, Eastern Arabic-Indic digits, ...
        EuropeanNumber: EN "European Number",

        /// Plus and minus signs
        // ///
        // /// ***General Scope***
        // ///
        // /// PLUS SIGN, MINUS SIGN
        EuropeanSeparator: ES "European Number Separator",

        /// A terminator in a numeric format context, includes currency signs
        // ///
        // /// ***General Scope***
        // ///
        // /// DEGREE SIGN, currency symbols, ...
        EuropeanTerminator: ET "European Number Terminator",

        /// Any Arabic-Indic digit
        // ///
        // /// ***General Scope***
        // ///
        // /// Arabic-Indic digits, Arabic decimal and thousands separators, ...
        ArabicNumber: AN "Arabic Number",

        /// Commas, colons, and slashes
        // ///
        // /// ***General Scope***
        // ///
        // /// COLON, COMMA, FULL STOP, NO_BREAK SPACE, ...
        CommonSeparator: CS "Common Number Separator",

        /// Any nonspacing mark
        // ///
        // /// ***General Scope***
        // ///
        // /// Characters with the General_Category values:
        // /// Mn (Nonspacing_Mark) and Me (Enclosing_Mark)
        NonspacingMark: NSM "Nonspacing Mark",

        /// Most format characters, control codes, or noncharacters
        // ///
        // /// ***General Scope***
        // ///
        // /// Default ignorables, non-characters, and control characters,
        // /// other than those explicitly given other types.
        BoundaryNeutral: BN "Boundary Neutral",

        // == Neutral == //

        /// Various newline characters
        // ///
        // /// ***General Scope***
        // ///
        // /// PARAGRAPH SEPARATOR, appropriate Newline Functions,
        // /// higher-level protocol paragraph determination
        ParagraphSeparator: B "Paragraph Separator",

        /// Various segment-related control codes
        // ///
        // /// ***General Scope***
        // ///
        // /// *Tab*
        SegmentSeparator: S "Segment Separator",

        /// Spaces
        // ///
        // /// ***General Scope***
        // ///
        // /// SPACE, FIGURE SPACE, LIN SEPARATOR, FORM FEED,
        // /// General Punctuation spaces, ...
        WhiteSpace: WS "Whitespace",

        /// Most other symbols and punctuation marks
        // ///
        // /// ***General Scope***
        // ///
        // /// All other characters, including OBJECT REPLACEMENT CHARACTER
        OtherNeutral: ON "Other Neutrals",

        // == Explicit Formatting == //

        /// U+202A: The LR embedding control
        LeftToRightEmbedding: LRE "Left-to-Right Embedding",

        /// U+202D: The LR override control
        LeftToRightOverride: LRO "Left-to-Right Override",

        /// U+202B: The RL embedding control
        RightToLeftEmbedding: RLE "Right-to-Left Embedding",

        /// U+202E: The RL override control
        RightToLeftOverride: RLO "Right-to-Left Override",

        /// U+202C: Terminates an embedding or override control
        PopDirectionalFormat: PDF "Pop Directional Format",

        /// U+2066: The LR isolate control
        LeftToRightIsolate: LRI "Left-to-Right Isolate",

        /// U+2067: The RL isolate control
        RightToLeftIsolate: RLI "Left-to-Right Isolate",

        /// U+2068: The first string isolate control
        FirstStrongIsolate: FSI "First Strong Isolate",

        /// U+2069: Terminates an isolate control
        PopDirectionalIsolate: PDI "Pop Directional Isolate",
    };

    /// Abbreviated name aliases for
    /// [*Bidi_Class*](http://www.unicode.org/reports/tr44/#Bidi_Class) property.
    ///
    /// <http://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt#Bidi_Class>
    pub mod abbr_names;
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
    /// Find the BidiClass of a single char.
    pub fn of(ch: char) -> BidiClass {
        bsearch_range_value_table(ch, BIDI_CLASS_TABLE)
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
