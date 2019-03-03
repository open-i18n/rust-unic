// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Types of Unicode Name Aliases
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum NameAliasType {
    /// Corrections for serious problems in the character names
    NameCorrections,

    /// ISO 6429 names for C0 and C1 control functions, and other
    /// commonly occurring names for control codes
    ControlCodeNames,

    /// A few widely used alternate names for format characters
    AlternateNames,

    /// Several documented labels for C1 control code points which
    /// were never actually approved in any standard
    Figments,

    /// Commonly occurring abbreviations (or acronyms) for control codes,
    /// format characters, spaces, and variation selectors
    NameAbbreviations,
}

impl NameAliasType {
    /// Find types of available Name Aliases for the given character
    pub fn of(ch: char) -> Option<&'static [NameAliasType]> {
        data::NAME_ALIAS_TYPES.find(ch)
    }
}

/// Find Name Aliases for the given character with the specified type
pub fn name_aliases_of(
    ch: char,
    name_alias_type: NameAliasType,
) -> Option<&'static [&'static str]> {
    match name_alias_type {
        NameAliasType::NameCorrections => data::CORRECTIONS.find(ch),
        NameAliasType::ControlCodeNames => data::CONTROLS.find(ch),
        NameAliasType::AlternateNames => data::ALTERNATES.find(ch),
        NameAliasType::Figments => data::FIGMENTS.find(ch),
        NameAliasType::NameAbbreviations => data::ABBREVIATIONS.find(ch),
    }
}

mod data {
    use unic_char_property::tables::CharDataTable;
    pub const CORRECTIONS: CharDataTable<&[&str]> = include!("../tables/corrections.rsv");
    pub const CONTROLS: CharDataTable<&[&str]> = include!("../tables/controls.rsv");
    pub const ALTERNATES: CharDataTable<&[&str]> = include!("../tables/alternates.rsv");
    pub const FIGMENTS: CharDataTable<&[&str]> = include!("../tables/figments.rsv");
    pub const ABBREVIATIONS: CharDataTable<&[&str]> = include!("../tables/abbreviations.rsv");

    // Bring all enum cases into scope, because NameAliasType is omitted
    // in name_alias_types.rsv to save space
    use crate::NameAliasType::{
        AlternateNames,
        ControlCodeNames,
        Figments,
        NameAbbreviations,
        NameCorrections,
    };
    pub const NAME_ALIAS_TYPES: CharDataTable<&[crate::NameAliasType]> =
        include!("../tables/name_alias_types.rsv");
}
