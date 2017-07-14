// Copyright 2012-2015 The Rust Project Developers.
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
//!

extern crate unic_ucd_core;


//use unic_ucd_core::UnicodeVersion;
//
//
///// The [Unicode version](http://www.unicode.org/versions/) of data
//pub const UNICODE_VERSION: UnicodeVersion = include!("tables/unicode_version.rsv");

/// Represents the Unicode Character
/// [*General Category*](http://unicode.org/reports/tr44/#General_Category) property.
///
/// * <http://unicode.org/reports/tr44/#General_Category_Values>
#[derive(Debug, PartialEq, Eq)]
pub enum GeneralCategory {
    /// An uppercase letter
    ///
    /// Abbreviated: Lu
    UppercaseLetter,

    /// A lowercase letter
    ///
    /// Abbreviated: Ll
    LowercaseLetter,

    /// A digraphic character, with first part uppercase
    ///
    /// Abbreviated: Lt
    TitlecaseLetter,

    /// A modifier letter
    ///
    /// Abbreviated: Lm
    ModifierLetter,

    /// Other letters, including syllables and ideographs
    ///
    /// Abbreviated: Lo
    OtherLetter,

    /// A nonspacing combining mark (zero advance width)
    ///
    /// Abbreviated: Mn
    NonspacingMark,

    /// A spacing combining mark (positive advance width)
    ///
    /// Abbreviated: Mc
    SpacingMark,

    /// An enclosing combining mark
    ///
    /// Abbreviated: Me
    EnclosingMark,

    /// A decimal digit
    ///
    /// Abbreviated: Nd
    DecimalNumber,

    /// A letterlike numeric character
    ///
    /// Abbreviated: Nl
    LetterNumber,

    /// A numeric character of other type
    ///
    /// Abbreviated: No
    OtherNumber,

    /// A connecting punctuation mark, like a tie
    ///
    /// Abbreviated: Pc
    ConnectorPunctuation,

    /// A dash or hyphen punctuation mark
    ///
    /// Abbreviated: Pd
    DashPunctuation,

    /// An opening punctuation mark (of a pair)
    ///
    /// Abbreviated: Ps
    OpenPunctuation,

    /// A closing punctuation mark (of a pair)
    ///
    /// Abbreviated: Pe
    ClosePunctuation,

    /// An initial quotation mark
    ///
    /// Abbreviated: Pi
    InitialPunctuation,

    /// A final quotation mark
    ///
    /// Abbreviated: Pf
    FinalPunctuation,

    /// A punctuation mark of other type
    ///
    /// Abbreviated: Po
    OtherPunctuation,

    /// A symbol of mathematical use
    ///
    /// Abbreviated: Sm
    MathSymbol,

    /// A currency sign
    ///
    /// Abbreviated: Sc
    CurrencySymbol,

    /// A non-letterlike modifier symbol
    ///
    /// Abbreviated: Sk
    ModifierSymbol,

    /// A symbol of other type
    ///
    /// Abbreviated: So
    OtherSymbol,

    /// A space character (of various non-zero widths)
    ///
    /// Abbreviated: Zs
    SpaceSeparator,

    /// U+2028 LINE SEPARATOR only
    ///
    /// Abbreviated: Zl
    LineSeparator,

    /// U+2029 PARAGRAPH SEPARATOR only
    ///
    /// Abbreviated: Zp
    ParagraphSeparator,

    /// A C0 or C1 control code
    ///
    /// Abbreviated: Cc
    Control,

    /// A format control character
    ///
    /// Abbreviated: Cf
    Format,

    /// A surrogate code point
    ///
    /// Abbreviated: Cs
    Surrogate,

    /// A private-use character
    ///
    /// Abbreviated: Co
    PrivateUse,

    /// Unassigned
    ///
    /// Abbreviated: Cn
    Unassigned,
}


use self::GeneralCategory::*;

// TODO: potentially change these to match pattern matching.
// Static array contains was easier and clearer to get the first draft up,
// but is it optimized to the same thing?
impl GeneralCategory {
    /// Lu | Ll | Lt
    ///
    /// Abbreviated: LC
    pub fn is_cased_letter(&self) -> bool {
        [UppercaseLetter, LowercaseLetter, TitlecaseLetter].contains(self)
    }

    /// Lu | Ll | Lt | Lm | Lo
    ///
    /// Abbreviated: L
    pub fn is_letter(&self) -> bool {
        [UppercaseLetter, LowercaseLetter, TitlecaseLetter, ModifierLetter, OtherLetter].contains(self)
    }

    /// Mn | Mc | Me
    ///
    /// Abbreviated: M
    pub fn is_mark(&self) -> bool {
        [NonspacingMark, SpacingMark, EnclosingMark].contains(self)
    }

    /// Nd | Nl | No
    ///
    /// Abbreviated: N
    pub fn is_number(&self) -> bool {
        [DecimalNumber, LetterNumber, OtherNumber].contains(self)
    }

    /// Pc | Pd | Ps | Pe | Pi | Pf | Po
    ///
    /// Abbreviated: P
    pub fn is_punctuation(&self) -> bool {
        [ConnectorPunctuation, DashPunctuation, OpenPunctuation, ClosePunctuation, InitialPunctuation, FinalPunctuation, OtherPunctuation].contains(self)
    }

    /// Sm | Sc | Sk | So
    ///
    /// Abbreviated: S
    pub fn is_symbol(&self) -> bool {
        [MathSymbol, CurrencySymbol, ModifierLetter, OtherSymbol].contains(self)
    }

    /// Zs | Zl | Zp
    ///
    /// Abbreviated: Z
    pub fn is_separator(&self) -> bool {
        [SpaceSeparator, LineSeparator, ParagraphSeparator].contains(self)
    }

    /// Cc | Cf | Cs | Co | Cn
    ///
    /// Abbreviated: C
    pub fn is_other(&self) -> bool {
        [Control, Format, Surrogate, PrivateUse, Unassigned].contains(self)
    }
}
