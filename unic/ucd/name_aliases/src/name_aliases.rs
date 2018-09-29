// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Corrections for serious problems in the character names
pub fn name_corrections_of(ch: char) -> Option<&'static [&'static str]> {
    data::CORRECTIONS.find(ch)
}

/// ISO 6429 names for C0 and C1 control functions, and other
/// commonly occurring names for control codes
pub fn control_code_names_of(ch: char) -> Option<&'static [&'static str]> {
    data::CONTROLS.find(ch)
}

/// A few widely used alternate names for format characters
pub fn alternate_names_of(ch: char) -> Option<&'static [&'static str]> {
    data::ALTERNATES.find(ch)
}

/// Several documented labels for C1 control code points which
/// were never actually approved in any standard
pub fn figments_of(ch: char) -> Option<&'static [&'static str]> {
    data::FIGMENTS.find(ch)
}

/// Commonly occurring abbreviations (or acronyms) for control codes,
/// format characters, spaces, and variation selectors
pub fn name_abbreviations_of(ch: char) -> Option<&'static [&'static str]> {
    data::ABBREVIATIONS.find(ch)
}

mod data {
    use unic_char_property::tables::CharDataTable;
    pub const CORRECTIONS: CharDataTable<&[&str]> = include!("../tables/corrections.rsv");
    pub const CONTROLS: CharDataTable<&[&str]> = include!("../tables/controls.rsv");
    pub const ALTERNATES: CharDataTable<&[&str]> = include!("../tables/alternates.rsv");
    pub const FIGMENTS: CharDataTable<&[&str]> = include!("../tables/figments.rsv");
    pub const ABBREVIATIONS: CharDataTable<&[&str]> = include!("../tables/abbreviations.rsv");
}
