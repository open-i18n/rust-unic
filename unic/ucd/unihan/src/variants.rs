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

pub fn simplified_variant_of(ch: char) -> Option<char> {
    data::SIMPLIFIED_VARIANT.find(ch)
}

pub fn traditional_variant_of(ch: char) -> Option<char> {
    data::TRADITIONAL_VARIANT.find(ch)
}

mod data {
    use unic_char_property::tables::CharDataTable;
    pub const SIMPLIFIED_VARIANT: CharDataTable<char> =
        include!("../tables/simplified_variant_map.rsv");
    pub const TRADITIONAL_VARIANT: CharDataTable<char> =
        include!("../tables/traditional_variant_map.rsv");
}
