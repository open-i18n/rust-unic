// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![deny(unsafe_code, missing_docs)]

//! # UNIC — UCD — Category
//!
//! A component of [`unic`: Unicode and Internationalization Crates for Rust](/unic/).
//!
//! Unicode General Category.
//!
//! > The General_Category property of a code point provides for the most general classification of
//! that code point. It is usually determined based on the primary characteristic of the assigned
//! character for that code point. For example, is the character a letter, a mark, a number,
//! punctuation, or a symbol, and if so, of what type? Other General_Category values define the
//! classification of code points which are not assigned to regular graphic characters, including
//! such statuses as private-use, control, surrogate code point, and reserved unassigned.
//!
//! > Many characters have multiple uses, and not all such cases can be captured entirely by the
//! General_Category value. For example, the General_Category value of Latin, Greek, or Hebrew
//! letters does not attempt to cover (or preclude) the numerical use of such letters as Roman
//! numerals or in other numerary systems. Conversely, the General_Category of ASCII digits 0..9 as
//! Nd (decimal digit) neither attempts to cover (or preclude) the occasional use of these digits as
//! letters in various orthographies. The General_Category is simply the first-order, most usual
//! categorization of a character.
//!
//! > For more information about the General_Category property, see Chapter 4,
//! Character Properties in [*Unicode*](http://unicode.org/reports/tr41/tr41-21.html#Unicode).
//!
//! -- [Unicode® Standard Annex #44 - Unicode Character Database](http://unicode.org/reports/tr44/)
//!

#[macro_use]
extern crate matches;
extern crate unic_ucd_core;

use std::cmp::Ordering;

use unic_ucd_core::UnicodeVersion;


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("tables/unicode_version.rsv");

/// Represents the Unicode Character
/// [*General Category*](http://unicode.org/reports/tr44/#General_Category) property.
///
/// This is a useful breakdown into various character types which can be used as a default
/// categorization in implementations. For the property values, see
/// [*General Category Values*](http://unicode.org/reports/tr44/#General_Category_Values).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeneralCategory {
    /// An uppercase letter (Short form: `Lu`)
    UppercaseLetter,

    /// A lowercase letter (Short form: `Ll`)
    LowercaseLetter,

    /// A digraphic character, with first part uppercase (Short form: `Lt`)
    TitlecaseLetter,

    /// A modifier letter (Short form: `Lm`)
    ModifierLetter,

    /// Other letters, including syllables and ideographs (Short form: `Lo`)
    OtherLetter,

    /// A nonspacing combining mark (zero advance width) (Short form: `Mn`)
    NonspacingMark,

    /// A spacing combining mark (positive advance width) (Short form: `Mc`)
    SpacingMark,

    /// An enclosing combining mark (Short form: `Me`)
    EnclosingMark,

    /// A decimal digit (Short form: `Nd`)
    DecimalNumber,

    /// A letterlike numeric character (Short form: `Nl`)
    LetterNumber,

    /// A numeric character of other type (Short form: `No`)
    OtherNumber,

    /// A connecting punctuation mark, like a tie (Short form: `Pc`)
    ConnectorPunctuation,

    /// A dash or hyphen punctuation mark (Short form: `Pd`)
    DashPunctuation,

    /// An opening punctuation mark (of a pair) (Short form: `Ps`)
    OpenPunctuation,

    /// A closing punctuation mark (of a pair) (Short form: `Pe`)
    ClosePunctuation,

    /// An initial quotation mark (Short form: `Pi`)
    InitialPunctuation,

    /// A final quotation mark (Short form: `Pf`)
    FinalPunctuation,

    /// A punctuation mark of other type (Short form: `Po`)
    OtherPunctuation,

    /// A symbol of mathematical use (Short form: `Sm`)
    MathSymbol,

    /// A currency sign (Short form: `Sc`)
    CurrencySymbol,

    /// A non-letterlike modifier symbol (Short form: `Sk`)
    ModifierSymbol,

    /// A symbol of other type (Short form: `So`)
    OtherSymbol,

    /// A space character (of various non-zero widths) (Short form: `Zs`)
    SpaceSeparator,

    /// U+2028 LINE SEPARATOR only (Short form: `Zl`)
    LineSeparator,

    /// U+2029 PARAGRAPH SEPARATOR only (Short form: `Zp`)
    ParagraphSeparator,

    /// A C0 or C1 control code (Short form: `Cc`)
    Control,

    /// A format control character (Short form: `Cf`)
    Format,

    /// A surrogate code point (Short form: `Cs`)
    Surrogate,

    /// A private-use character (Short form: `Co`)
    PrivateUse,

    /// Unassigned (Short form: `Cn`)
    Unassigned,
}


use self::GeneralCategory::*;

impl GeneralCategory {
    /// `Lu` | `Ll` | `Lt`  (Short form: `LC`)
    pub fn is_cased_letter(&self) -> bool {
        matches!(*self, UppercaseLetter | LowercaseLetter | TitlecaseLetter)
    }

    /// `Lu` | `Ll` | `Lt` | `Lm` | `Lo`  (Short form: `L`)
    pub fn is_letter(&self) -> bool {
        matches!(
            *self,
            UppercaseLetter | LowercaseLetter | TitlecaseLetter | ModifierLetter | OtherLetter
        )
    }

    /// `Mn` | `Mc` | `Me`  (Short form: `M`)
    pub fn is_mark(&self) -> bool {
        matches!(*self, NonspacingMark | SpacingMark | EnclosingMark)
    }

    /// `Nd` | `Nl` | `No`  (Short form: `N`)
    pub fn is_number(&self) -> bool {
        matches!(*self, DecimalNumber | LetterNumber | OtherNumber)
    }

    /// `Pc` | `Pd` | `Ps` | `Pe` | `Pi` | `Pf` | `Po`  (Short form: `P`)
    pub fn is_punctuation(&self) -> bool {
        matches!(
            *self,
            ConnectorPunctuation | DashPunctuation | OpenPunctuation | ClosePunctuation |
                InitialPunctuation | FinalPunctuation | OtherPunctuation
        )
    }

    /// `Sm` | `Sc` | `Sk` | `So`  (Short form: `S`)
    pub fn is_symbol(&self) -> bool {
        matches!(
            *self,
            MathSymbol | CurrencySymbol | ModifierLetter | OtherSymbol
        )
    }

    /// `Zs` | `Zl` | `Zp`  (Short form: `Z`)
    pub fn is_separator(&self) -> bool {
        matches!(*self, SpaceSeparator | LineSeparator | ParagraphSeparator)
    }

    /// `Cc` | `Cf` | `Cs` | `Co` | `Cn`  (Short form: `C`)
    pub fn is_other(&self) -> bool {
        matches!(
            *self,
            Control | Format | Surrogate | PrivateUse | Unassigned
        )
    }
}

const GENERAL_CATEGORY_TABLE: &'static [(char, char, GeneralCategory)] =
    include!("tables/general_category.rsv");

impl GeneralCategory {
    /// Find the GeneralCategory of a single char.
    pub fn of(ch: char) -> GeneralCategory {
        bsearch_range_value_table(ch, GENERAL_CATEGORY_TABLE)
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
}
