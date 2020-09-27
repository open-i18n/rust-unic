// Copyright 2016 The rust-url developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Represents the IDNA Mapping status of the Unicode character.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mapping {
    /// Valid, and not modified.
    Valid,

    /// Removed from the string.
    Ignored,

    /// Replaced in the string.
    Mapped(&'static str),

    /// Valid if *Nontransitional Processing*, or Mapped if Transitional Processing.
    Deviation(&'static str),

    /// Not allowed, result in error.
    Disallowed,

    /// Disallowed if *UseSTD3ASCIIRules* flag is set, Valid otherwise.
    DisallowedStd3Valid,

    /// Disallowed if *UseSTD3ASCIIRules* flag is set, Mapped otherwise.
    DisallowedStd3Mapped(&'static str),
}

mod data {
    use super::Mapping::*;
    use unic_char_property::tables::CharDataTable;

    #[allow(clippy::unreadable_literal)]
    pub const MAPPING: CharDataTable<super::Mapping> = include!("../tables/idna_mapping.rsv");
}

impl Mapping {
    /// Get Mapping status of the character.
    pub fn of(ch: char) -> Mapping {
        data::MAPPING.find(ch).expect("Table is missing value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping() {
        use crate::Mapping::*;

        assert_eq!(Mapping::of('\u{0}'), DisallowedStd3Valid);
        assert_eq!(Mapping::of('-'), Valid);
        assert_eq!(Mapping::of('A'), Mapped("a"));
        assert_eq!(Mapping::of('\u{80}'), Disallowed);
        assert_eq!(Mapping::of('\u{a0}'), DisallowedStd3Mapped(" "));
        assert_eq!(Mapping::of('\u{ad}'), Ignored);
        assert_eq!(Mapping::of('\u{200c}'), Deviation(""));
    }
}
