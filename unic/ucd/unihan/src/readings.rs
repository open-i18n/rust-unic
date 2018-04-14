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

pub fn definition_of(ch: char) -> Option<&'static str> {
    data::DEFINITIONS.find(ch)
}

pub fn mandarin_of(ch: char) -> Option<&'static str> {
    // TODO: When there are two values, then the first is preferred for
    // zh-Hans (CN) and the second is preferred for zh-Hant (TW).
    data::MANDARINS.find(ch)
}

mod data {
    use unic_char_property::tables::CharDataTable;
    pub const DEFINITIONS: CharDataTable<&str> = include!("../tables/definition_map.rsv");
    pub const MANDARINS: CharDataTable<&str> = include!("../tables/mandarin_map.rsv");
}
