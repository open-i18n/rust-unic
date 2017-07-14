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
use std::fmt;


/// Represents the Unicode character
/// [*Bidi_Class*](http://www.unicode.org/reports/tr44/#Bidi_Class) property, also known as the
/// *bidirectional character type*.
///
/// * <http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types>
/// * <http://www.unicode.org/reports/tr44/#Bidi_Class_Values>
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
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

    /// Human-readable description of the Bidi Class property values
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
            L | R | AL => Strong,
            EN | ES | ET | AN | CS | NSM | BN => Weak,
            B | S | WS | ON => Neutral,
            LRE | LRO | RLE | RLO | PDF | LRI | RLI | FSI | PDI => ExplicitFormatting,
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

impl fmt::Display for BidiClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}


#[cfg(test)]
mod tests {
    use super::BidiClass;
    use BidiClass::*;

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
        assert_eq!(format!("{}", BidiClass::L), "Left-to-Right");
        assert_eq!(format!("{}", BidiClass::R), "Right-to-Left");
        assert_eq!(format!("{}", BidiClass::AL), "Right-to-Left Arabic");
        assert_eq!(format!("{}", BidiClass::FSI), "First Strong Isolate");
    }
}
