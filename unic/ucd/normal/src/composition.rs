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

use unic_char_property::tables::CharDataTable;

pub mod data {
    use crate::decomposition_type::long_names::*;
    use crate::DecompositionType;
    use unic_char_property::tables::CharDataTable;

    pub const CANONICAL_COMPOSITION_MAPPING: CharDataTable<CharDataTable<char>> =
        include!("../tables/canonical_composition_mapping.rsv");

    pub const CANONICAL_DECOMPOSITION_MAPPING: CharDataTable<&[char]> =
        include!("../tables/canonical_decomposition_mapping.rsv");

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub const COMPATIBILITY_DECOMPOSITION_MAPPING: CharDataTable<(DecompositionType, &[char])> =
        include!("../tables/compatibility_decomposition_mapping.rsv");
}

/// Canonical Composition of the character.
pub fn canonical_composition(c: char) -> Option<CharDataTable<char>> {
    data::CANONICAL_COMPOSITION_MAPPING.find(c)
}

/// Canonical Decomposition of the character.
pub fn canonical_decomposition(c: char) -> Option<&'static [char]> {
    data::CANONICAL_DECOMPOSITION_MAPPING.find(c)
}

/// Compatibility Decomposition of the character.
pub fn compatibility_decomposition(c: char) -> Option<&'static [char]> {
    data::COMPATIBILITY_DECOMPOSITION_MAPPING
        .find(c)
        .map(|it| it.1)
}
