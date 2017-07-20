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
    /// Represents the Unicode Character
    /// [*General Category*](http://unicode.org/reports/tr44/#General_Category) property.
    ///
    /// This is a useful breakdown into various character types which can be used as a default
    /// categorization in implementations. For the property values, see
    /// [*General Category Values*](http://unicode.org/reports/tr44/#General_Category_Values).
    pub enum GeneralCategory {
        /// An uppercase letter
        UppercaseLetter: Lu "Letter, uppercase",

        /// A lowercase letter
        LowercaseLetter: Ll "Letter, lowercase",

        /// A digraphic character, with first part uppercase
        TitlecaseLetter: Lt "Letter, titlecase",

        /// A modifier letter
        ModifierLetter: Lm "Letter, modifier",

        /// Other letters, including syllables and ideographs
        OtherLetter: Lo "Letter, other",

        /// A nonspacing combining mark (zero advance width)
        NonspacingMark: Mn "Mark, nonspacing",

        /// A spacing combining mark (positive advance width)
        SpacingMark: Mc "Mark, spacing combining",

        /// An enclosing combining mark
        EnclosingMark: Me "Mark, enclosing",

        /// A decimal digit
        DecimalNumber: Nd "Number, decimal digit",

        /// A letterlike numeric character
        LetterNumber: Nl "Number, letter",

        /// A numeric character of other type
        OtherNumber: No "Number, other",

        /// A connecting punctuation mark, like a tie
        ConnectorPunctuation: Pc "Punctuation, connector",

        /// A dash or hyphen punctuation mark
        DashPunctuation: Pd "Punctuation, dash",

        /// An opening punctuation mark (of a pair)
        OpenPunctuation: Ps "Punctuation, dash",

        /// A closing punctuation mark (of a pair)
        ClosePunctuation: Pe "Punctuation, close",

        /// An initial quotation mark
        InitialPunctuation: Pi "Punctuation, initial quote",

        /// A final quotation mark
        FinalPunctuation: Pf "Punctuation, final quote",

        /// A punctuation mark of other type
        OtherPunctuation: Po "Punctuation, other",

        /// A symbol of mathematical use
        MathSymbol: Sm "Symbol, math",

        /// A currency sign
        CurrencySymbol: Sc "Symbol, currency",

        /// A non-letterlike modifier symbol
        ModifierSymbol: Sk "Symbol, modifier",

        /// A symbol of other type
        OtherSymbol: So "Symbol, other",

        /// A space character (of various non-zero widths)
        SpaceSeparator: Zs "Separator, space",

        /// U+2028 LINE SEPARATOR only
        LineSeparator: Zl "Separator, line",

        /// U+2029 PARAGRAPH SEPARATOR only
        ParagraphSeparator: Zp "Separator, paragraph",

        /// A C0 or C1 control code
        Control: Cc "Other, Control",

        /// A format control character
        Format: Cf "Other, format",

        /// A surrogate code point
        Surrogate: Cs "Other, surrogate",

        /// A private-use character
        PrivateUse: Co "Other, private use",

        /// Unassigned
        Unassigned: Cn "Other, not assigned",
    };

    /// Abbreviated name aliases for the
    /// [*General Category*](http://unicode.org/reports/tr44/#General_Category) property.
    ///
    /// <http://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt#Genral_Property>
    pub mod abbr_names;
}

use self::GeneralCategory::*;
use self::abbr_names::*;

const GENERAL_CATEGORY_TABLE: &'static [(char, char, GeneralCategory)] =
    include!("tables/general_category.rsv");

impl GeneralCategory {
    /// Find the GeneralCategory of a single char.
    pub fn of(ch: char) -> GeneralCategory {
        bsearch_range_value_table(ch, GENERAL_CATEGORY_TABLE)
    }
}

impl GeneralCategory {
    /// `Lu` | `Ll` | `Lt`  (Short form: `LC`)
    pub fn is_cased_letter(&self) -> bool {
        matches!(*self, Lu | Ll | Lt)
    }

    /// `Lu` | `Ll` | `Lt` | `Lm` | `Lo`  (Short form: `L`)
    pub fn is_letter(&self) -> bool {
        matches!(*self, Lu | Ll | Lt | Lm | Lo)
    }

    /// `Mn` | `Mc` | `Me`  (Short form: `M`)
    pub fn is_mark(&self) -> bool {
        matches!(*self, Mn | Mc | Me)
    }

    /// `Nd` | `Nl` | `No`  (Short form: `N`)
    pub fn is_number(&self) -> bool {
        matches!(*self, Nd | Nl | No)
    }

    /// `Pc` | `Pd` | `Ps` | `Pe` | `Pi` | `Pf` | `Po`  (Short form: `P`)
    pub fn is_punctuation(&self) -> bool {
        matches!(*self, Pc | Pd | Ps | Pe | Pi | Pf | Po)
    }

    /// `Sm` | `Sc` | `Sk` | `So`  (Short form: `S`)
    pub fn is_symbol(&self) -> bool {
        matches!(*self, Sm | Sc | Sk | So)
    }

    /// `Zs` | `Zl` | `Zp`  (Short form: `Z`)
    pub fn is_separator(&self) -> bool {
        matches!(*self, Zs | Zl | Zp)
    }

    /// `Cc` | `Cf` | `Cs` | `Co` | `Cn`  (Short form: `C`)
    pub fn is_other(&self) -> bool {
        matches!(*self, Cc | Cf | Cs | Co | Cn)
    }
}

fn bsearch_range_value_table(
    c: char,
    r: &'static [(char, char, GeneralCategory)],
) -> GeneralCategory {
    match r.binary_search_by(|&(lo, hi, _)| if lo <= c && c <= hi {
        Ordering::Equal
    } else if hi < c {
        Ordering::Less
    } else {
        Ordering::Greater
    }) {
        Ok(idx) => {
            let (_, _, category) = r[idx];
            category
        }
        Err(_) => GeneralCategory::Unassigned,
    }
}

#[cfg(test)]
mod tests {
    use super::GeneralCategory as GC;
    use std::char;

    #[test]
    fn test_ascii() {
        for c in 0x00..(0x1F + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::Control);
        }

        assert_eq!(GC::of(' '), GC::SpaceSeparator);
        assert_eq!(GC::of('!'), GC::OtherPunctuation);
        assert_eq!(GC::of('"'), GC::OtherPunctuation);
        assert_eq!(GC::of('#'), GC::OtherPunctuation);
        assert_eq!(GC::of('$'), GC::CurrencySymbol);
        assert_eq!(GC::of('%'), GC::OtherPunctuation);
        assert_eq!(GC::of('&'), GC::OtherPunctuation);
        assert_eq!(GC::of('\''), GC::OtherPunctuation);
        assert_eq!(GC::of('('), GC::OpenPunctuation);
        assert_eq!(GC::of(')'), GC::ClosePunctuation);
        assert_eq!(GC::of('*'), GC::OtherPunctuation);
        assert_eq!(GC::of('+'), GC::MathSymbol);
        assert_eq!(GC::of(','), GC::OtherPunctuation);
        assert_eq!(GC::of('-'), GC::DashPunctuation);
        assert_eq!(GC::of('.'), GC::OtherPunctuation);
        assert_eq!(GC::of('/'), GC::OtherPunctuation);

        for c in ('0' as u32)..('9' as u32 + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::DecimalNumber);
        }

        assert_eq!(GC::of(':'), GC::OtherPunctuation);
        assert_eq!(GC::of(';'), GC::OtherPunctuation);
        assert_eq!(GC::of('<'), GC::MathSymbol);
        assert_eq!(GC::of('='), GC::MathSymbol);
        assert_eq!(GC::of('>'), GC::MathSymbol);
        assert_eq!(GC::of('?'), GC::OtherPunctuation);
        assert_eq!(GC::of('@'), GC::OtherPunctuation);

        for c in ('A' as u32)..('Z' as u32 + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::UppercaseLetter);
        }

        assert_eq!(GC::of('['), GC::OpenPunctuation);
        assert_eq!(GC::of('\\'), GC::OtherPunctuation);
        assert_eq!(GC::of(']'), GC::ClosePunctuation);
        assert_eq!(GC::of('^'), GC::ModifierSymbol);
        assert_eq!(GC::of('_'), GC::ConnectorPunctuation);
        assert_eq!(GC::of('`'), GC::ModifierSymbol);

        for c in ('a' as u32)..('z' as u32 + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::LowercaseLetter);
        }

        assert_eq!(GC::of('{'), GC::OpenPunctuation);
        assert_eq!(GC::of('|'), GC::MathSymbol);
        assert_eq!(GC::of('}'), GC::ClosePunctuation);
        assert_eq!(GC::of('~'), GC::MathSymbol);
    }

    #[test]
    fn test_bmp_edge() {
        // 0xFEFF ZERO WIDTH NO-BREAK SPACE (or) BYTE ORDER MARK
        let bom = char::from_u32(0xFEFF).unwrap();
        assert_eq!(GC::of(bom), GC::Format);
        // 0xFFFC OBJECT REPLACEMENT CHARACTER
        assert_eq!(GC::of('￼'), GC::OtherSymbol);
        // 0xFFFD REPLACEMENT CHARACTER
        assert_eq!(GC::of('�'), GC::OtherSymbol);

        for &c in [0xFFEF, 0xFFFE, 0xFFFF].iter() {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::Unassigned);
        }
    }

    #[test]
    fn test_private_use() {
        for c in 0xF0000..(0xFFFFD + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::PrivateUse);
        }

        for c in 0x100000..(0x10FFFD + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::PrivateUse);
        }

        for &c in [0xFFFFE, 0xFFFFF, 0x10FFFE, 0x10FFFF].iter() {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::Unassigned);
        }
    }
}
