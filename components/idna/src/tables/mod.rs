// Copyright 2013-2014 The rust-url developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use unic_ucd_core::UnicodeVersion;

use std::cmp::Ordering;


/// The version of [Unicode IDNA Compatibility Processing](http://www.unicode.org/reports/tr46/)
pub const UNICODE_VERSION: UnicodeVersion = include!("unicode_version.rsv");


#[repr(u8)]
#[derive(Debug)]
pub enum Mapping {
    Valid,
    Ignored,
    Mapped(StringTableSlice),
    Deviation(StringTableSlice),
    Disallowed,
    DisallowedStd3Valid,
    DisallowedStd3Mapped(StringTableSlice),
}

struct Range {
    from: char,
    to: char,
    mapping: Mapping,
}

#[derive(Debug)]
pub struct StringTableSlice {
    byte_start: u16,
    byte_len: u16,
}


// Used in idna_map.rsv
use self::Mapping::*;


static MAP: &'static [Range] = include!("idna_map.rsv");
static MAP_STRING: &'static str = include!("idna_map_string.rsv");


pub fn decode_slice(slice: &StringTableSlice) -> &'static str {
    let start = slice.byte_start as usize;
    let len = slice.byte_len as usize;
    &MAP_STRING[start..(start + len)]
}

fn find_char(codepoint: char) -> &'static Mapping {
    let r = TABLE.binary_search_by(|ref range| if codepoint > range.to {
        Ordering::Less
    } else if codepoint < range.from {
        Ordering::Greater
    } else {
        Ordering::Equal
    });
    r.ok().map(|i| &TABLE[i].mapping).unwrap()
}
