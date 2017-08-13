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
use unic_char_property::{CharProperty, CompleteCharProperty, EnumeratedCharProperty};


/// Represents the Unicode Character
/// [*General_Category*](http://unicode.org/reports/tr44/#General_Category) property.
///
/// This is a useful breakdown into various character types which can be used as a default
/// categorization in implementations. For the property values, see
/// [*General_Category Values*](http://unicode.org/reports/tr44/#General_Category_Values).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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


impl CharProperty for GeneralCategory {
    fn prop_abbr_name() -> &'static str {
        "gc"
    }

    fn prop_long_name() -> &'static str {
        "General_Category"
    }

    fn prop_human_name() -> &'static str {
        "General Category"
    }
}


impl CompleteCharProperty for GeneralCategory {
    fn of(ch: char) -> Self {
        Self::of(ch)
    }
}


impl EnumeratedCharProperty for GeneralCategory {
    fn all_values() -> &'static [Self] {
        Self::all_values()
    }

    fn abbr_name(&self) -> &'static str {
        self.abbr_name()
    }

    fn long_name(&self) -> &'static str {
        self.long_name()
    }

    fn human_name(&self) -> &'static str {
        self.human_name()
    }
}


pub mod abbr_names {
    pub use GeneralCategory::UppercaseLetter as Lu;
    pub use GeneralCategory::LowercaseLetter as Ll;
    pub use GeneralCategory::TitlecaseLetter as Lt;
    pub use GeneralCategory::ModifierLetter as Lm;
    pub use GeneralCategory::OtherLetter as Lo;
    pub use GeneralCategory::NonspacingMark as Mn;
    pub use GeneralCategory::SpacingMark as Mc;
    pub use GeneralCategory::EnclosingMark as Me;
    pub use GeneralCategory::DecimalNumber as Nd;
    pub use GeneralCategory::LetterNumber as Nl;
    pub use GeneralCategory::OtherNumber as No;
    pub use GeneralCategory::ConnectorPunctuation as Pc;
    pub use GeneralCategory::DashPunctuation as Pd;
    pub use GeneralCategory::OpenPunctuation as Ps;
    pub use GeneralCategory::ClosePunctuation as Pe;
    pub use GeneralCategory::InitialPunctuation as Pi;
    pub use GeneralCategory::FinalPunctuation as Pf;
    pub use GeneralCategory::OtherPunctuation as Po;
    pub use GeneralCategory::MathSymbol as Sm;
    pub use GeneralCategory::CurrencySymbol as Sc;
    pub use GeneralCategory::ModifierSymbol as Sk;
    pub use GeneralCategory::OtherSymbol as So;
    pub use GeneralCategory::SpaceSeparator as Zs;
    pub use GeneralCategory::LineSeparator as Zl;
    pub use GeneralCategory::ParagraphSeparator as Zp;
    pub use GeneralCategory::Control as Cc;
    pub use GeneralCategory::Format as Cf;
    pub use GeneralCategory::Surrogate as Cs;
    pub use GeneralCategory::PrivateUse as Co;
    pub use GeneralCategory::Unassigned as Cn;
}
use self::abbr_names::*;

const GENERAL_CATEGORY_TABLE: &'static [(char, char, GeneralCategory)] =
    include!("tables/general_category.rsv");

impl GeneralCategory {
    /// Find the `GeneralCategory` of a single char.
    pub fn of(ch: char) -> GeneralCategory {
        *GENERAL_CATEGORY_TABLE.find_or(ch, &GeneralCategory::Unassigned)
    }

    /// Exhaustive list of all `GeneralCategory` property values.
    pub fn all_values() -> &'static [GeneralCategory] {
        const ALL_VALUES: &[GeneralCategory] = &[
            GeneralCategory::UppercaseLetter,
            GeneralCategory::LowercaseLetter,
            GeneralCategory::TitlecaseLetter,
            GeneralCategory::ModifierLetter,
            GeneralCategory::OtherLetter,
            GeneralCategory::NonspacingMark,
            GeneralCategory::SpacingMark,
            GeneralCategory::EnclosingMark,
            GeneralCategory::DecimalNumber,
            GeneralCategory::LetterNumber,
            GeneralCategory::OtherNumber,
            GeneralCategory::ConnectorPunctuation,
            GeneralCategory::DashPunctuation,
            GeneralCategory::OpenPunctuation,
            GeneralCategory::ClosePunctuation,
            GeneralCategory::InitialPunctuation,
            GeneralCategory::FinalPunctuation,
            GeneralCategory::OtherPunctuation,
            GeneralCategory::MathSymbol,
            GeneralCategory::CurrencySymbol,
            GeneralCategory::ModifierSymbol,
            GeneralCategory::OtherSymbol,
            GeneralCategory::SpaceSeparator,
            GeneralCategory::LineSeparator,
            GeneralCategory::ParagraphSeparator,
            GeneralCategory::Control,
            GeneralCategory::Format,
            GeneralCategory::Surrogate,
            GeneralCategory::PrivateUse,
            GeneralCategory::Unassigned,
        ];
        ALL_VALUES
    }

    /// The *abbreviated name* of the property value.
    pub fn abbr_name(&self) -> &'static str {
        match *self {
            GeneralCategory::UppercaseLetter => "Lu",
            GeneralCategory::LowercaseLetter => "Ll",
            GeneralCategory::TitlecaseLetter => "Lt",
            GeneralCategory::ModifierLetter => "Lm",
            GeneralCategory::OtherLetter => "Lo",
            GeneralCategory::NonspacingMark => "Mn",
            GeneralCategory::SpacingMark => "Mc",
            GeneralCategory::EnclosingMark => "Me",
            GeneralCategory::DecimalNumber => "Nd",
            GeneralCategory::LetterNumber => "Nl",
            GeneralCategory::OtherNumber => "No",
            GeneralCategory::ConnectorPunctuation => "Pc",
            GeneralCategory::DashPunctuation => "Pd",
            GeneralCategory::OpenPunctuation => "Ps",
            GeneralCategory::ClosePunctuation => "Pe",
            GeneralCategory::InitialPunctuation => "Pi",
            GeneralCategory::FinalPunctuation => "Pf",
            GeneralCategory::OtherPunctuation => "Po",
            GeneralCategory::MathSymbol => "Sm",
            GeneralCategory::CurrencySymbol => "Sc",
            GeneralCategory::ModifierSymbol => "Sk",
            GeneralCategory::OtherSymbol => "So",
            GeneralCategory::SpaceSeparator => "Zs",
            GeneralCategory::LineSeparator => "Zl",
            GeneralCategory::ParagraphSeparator => "Zp",
            GeneralCategory::Control => "Cc",
            GeneralCategory::Format => "Cf",
            GeneralCategory::Surrogate => "Cs",
            GeneralCategory::PrivateUse => "Co",
            GeneralCategory::Unassigned => "Cn",
        }
    }

    /// The *long name* of the property value.
    pub fn long_name(&self) -> &'static str {
        match *self {
            GeneralCategory::UppercaseLetter => "Uppercase_Letter",
            GeneralCategory::LowercaseLetter => "Lowercase_Letter",
            GeneralCategory::TitlecaseLetter => "Titlecase_Letter",
            GeneralCategory::ModifierLetter => "Modifier_Letter",
            GeneralCategory::OtherLetter => "Other_Letter",
            GeneralCategory::NonspacingMark => "Nonspacing_Mark",
            GeneralCategory::SpacingMark => "Spacing_Mark",
            GeneralCategory::EnclosingMark => "Enclosing_Mark",
            GeneralCategory::DecimalNumber => "Decimal_Number",
            GeneralCategory::LetterNumber => "Letter_Number",
            GeneralCategory::OtherNumber => "Other_Number",
            GeneralCategory::ConnectorPunctuation => "Connector_Punctuation",
            GeneralCategory::DashPunctuation => "Dash_Punctuation",
            GeneralCategory::OpenPunctuation => "Open_Punctuation",
            GeneralCategory::ClosePunctuation => "Close_Punctuation",
            GeneralCategory::InitialPunctuation => "Initial_Punctuation",
            GeneralCategory::FinalPunctuation => "Final_Punctuation",
            GeneralCategory::OtherPunctuation => "Other_Punctuation",
            GeneralCategory::MathSymbol => "Math_Symbol",
            GeneralCategory::CurrencySymbol => "Currency_Symbol",
            GeneralCategory::ModifierSymbol => "Modifier_Symbol",
            GeneralCategory::OtherSymbol => "Other_Symbol",
            GeneralCategory::SpaceSeparator => "Space_Separator",
            GeneralCategory::LineSeparator => "Line_Separator",
            GeneralCategory::ParagraphSeparator => "Paragraph_Separator",
            GeneralCategory::Control => "Control",
            GeneralCategory::Format => "Format",
            GeneralCategory::Surrogate => "Surrogate",
            GeneralCategory::PrivateUse => "Private_Use",
            GeneralCategory::Unassigned => "Unassigned",
        }
    }

    /// The *human-readable name* of the property value.
    #[inline]
    pub fn human_name(&self) -> &'static str {
        match *self {
            GeneralCategory::UppercaseLetter => "Uppercase Letter",
            GeneralCategory::LowercaseLetter => "Lowercase Letter",
            GeneralCategory::TitlecaseLetter => "Titlecase Letter",
            GeneralCategory::ModifierLetter => "Modifier Letter",
            GeneralCategory::OtherLetter => "Other Letter",
            GeneralCategory::NonspacingMark => "Nonspacing Mark",
            GeneralCategory::SpacingMark => "Spacing Mark",
            GeneralCategory::EnclosingMark => "Enclosing Mark",
            GeneralCategory::DecimalNumber => "Decimal Number",
            GeneralCategory::LetterNumber => "Letter Number",
            GeneralCategory::OtherNumber => "Other Number",
            GeneralCategory::ConnectorPunctuation => "Connector Punctuation",
            GeneralCategory::DashPunctuation => "Dash Punctuation",
            GeneralCategory::OpenPunctuation => "Open Punctuation",
            GeneralCategory::ClosePunctuation => "Close Punctuation",
            GeneralCategory::InitialPunctuation => "Initial Punctuation",
            GeneralCategory::FinalPunctuation => "Final Punctuation",
            GeneralCategory::OtherPunctuation => "Other Punctuation",
            GeneralCategory::MathSymbol => "Math Symbol",
            GeneralCategory::CurrencySymbol => "Currency Symbol",
            GeneralCategory::ModifierSymbol => "Modifier Symbol",
            GeneralCategory::OtherSymbol => "Other Symbol",
            GeneralCategory::SpaceSeparator => "Space Separator",
            GeneralCategory::LineSeparator => "Line Separator",
            GeneralCategory::ParagraphSeparator => "Paragraph Separator",
            GeneralCategory::Control => "Control",
            GeneralCategory::Format => "Format",
            GeneralCategory::Surrogate => "Surrogate",
            GeneralCategory::PrivateUse => "Private Use",
            GeneralCategory::Unassigned => "Unassigned",
        }
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


impl Default for GeneralCategory {
    fn default() -> Self {
        GeneralCategory::Unassigned
    }
}


impl fmt::Display for GeneralCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.human_name())
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
        for c in 0xF_0000..(0xF_FFFD + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::PrivateUse);
        }

        for c in 0x10_0000..(0x10_FFFD + 1) {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::PrivateUse);
        }

        for &c in [0xF_FFFE, 0xF_FFFF, 0x10_FFFE, 0x10_FFFF].iter() {
            let c = char::from_u32(c).unwrap();
            assert_eq!(GC::of(c), GC::Unassigned);
        }
    }

    #[test]
    fn test_abbr_name() {
        assert_eq!(GC::UppercaseLetter.abbr_name(), "Lu");
        assert_eq!(GC::Unassigned.abbr_name(), "Cn");
    }

    #[test]
    fn test_long_name() {
        assert_eq!(GC::UppercaseLetter.long_name(), "Uppercase_Letter");
        assert_eq!(GC::Unassigned.long_name(), "Unassigned");
    }

    #[test]
    fn test_human_name() {
        assert_eq!(GC::UppercaseLetter.human_name(), "Uppercase Letter");
        assert_eq!(GC::Unassigned.human_name(), "Unassigned");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", GC::UppercaseLetter), "Uppercase Letter");
        assert_eq!(format!("{}", GC::Unassigned), "Unassigned");
    }
}
