// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! *Indic_Conjunct_Break* Character Property.

use unic_char_property::TotalCharProperty;

char_property! {
    /// Represents the Unicode character
    /// [`Indic_Conjunct_Break`](https://www.unicode.org/reports/tr44/#Indic_Conjunct_Break)
    /// property, used by rule GB9c of the grapheme cluster boundary algorithm.
    ///
    /// ## References
    ///
    /// * <https://www.unicode.org/reports/tr44/#Indic_Conjunct_Break>
    /// * <https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries>
    pub enum IndicConjunctBreak {
        abbr => "InCB";
        long => "Indic_Conjunct_Break";
        human => "Indic Conjunct Break";

        /// All other characters.
        None {
            abbr => None,
            long => None,
            human => "None",
        }

        /// ```text
        /// Indic_Syllabic_Category = Consonant, and
        /// Script = Bengali, Devanagari, Gujarati, Malayalam, Oriya, or Telugu
        /// ```
        Consonant {
            abbr => Consonant,
            long => Consonant,
            human => "Consonant",
        }

        /// ```text
        /// Grapheme_Cluster_Break = Extend or ZWJ, and
        /// not Indic_Conjunct_Break = Linker,
        /// with some exceptions
        /// ```
        Extend {
            abbr => Extend,
            long => Extend,
            human => "Extend",
        }

        /// ```text
        /// Indic_Syllabic_Category = Virama or Invisible_Stacker, and
        /// Script = Bengali, Devanagari, Gujarati, Malayalam, Oriya, or Telugu
        /// ```
        Linker {
            abbr => Linker,
            long => Linker,
            human => "Linker",
        }
    }

    /// Abbreviated name aliases for the
    /// [`Indic_Conjunct_Break`](https://www.unicode.org/reports/tr44/#Indic_Conjunct_Break)
    /// property.
    pub mod abbr_names for abbr;

    /// Long name aliases for the
    /// [`Indic_Conjunct_Break`](https://www.unicode.org/reports/tr44/#Indic_Conjunct_Break)
    /// property.
    pub mod long_names for long;
}

impl TotalCharProperty for IndicConjunctBreak {
    fn of(ch: char) -> Self {
        Self::of(ch)
    }
}

impl Default for IndicConjunctBreak {
    fn default() -> Self {
        IndicConjunctBreak::None
    }
}

mod data {
    use super::long_names as InCB;
    use unic_char_property::tables::CharDataTable;
    pub const INDIC_CONJUNCT_BREAK_TABLE: CharDataTable<super::IndicConjunctBreak> =
        include!("../tables/indic_conjunct_break.rsv");
}

impl IndicConjunctBreak {
    /// Find the character `Indic_Conjunct_Break` property value.
    pub fn of(ch: char) -> IndicConjunctBreak {
        data::INDIC_CONJUNCT_BREAK_TABLE.find_or_default(ch)
    }
}

#[cfg(test)]
mod tests {
    use super::IndicConjunctBreak as InCB;
    use unic_char_property::EnumeratedCharProperty;

    #[test]
    fn test_values() {
        assert_eq!(InCB::of('\u{0000}'), InCB::None);
        assert_eq!(InCB::of('\u{0041}'), InCB::None);
        assert_eq!(InCB::of('\u{0300}'), InCB::Extend);
        assert_eq!(InCB::of('\u{0915}'), InCB::Consonant);
        assert_eq!(InCB::of('\u{094D}'), InCB::Linker);
        assert_eq!(InCB::of('\u{1CF5}'), InCB::Linker);
        assert_eq!(InCB::of('\u{200D}'), InCB::Extend);
        assert_eq!(InCB::of('\u{10FFFF}'), InCB::None);
    }

    #[test]
    fn test_abbr_name() {
        assert_eq!(InCB::None.abbr_name(), "None");
        assert_eq!(InCB::Consonant.abbr_name(), "Consonant");
        assert_eq!(InCB::Linker.abbr_name(), "Linker");
    }
}
