// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod numeric_values;
pub mod readings;
pub mod variants;

use std::char;
use std::collections::BTreeMap;

use regex::Regex;

lazy_static! {
    pub static ref UNIHAN_DATA_ENTRY_REGEX: Regex = Regex::new(
        r"(?xm)^ # every line
          U\+([[:xdigit:]]{4,6}) # [1]codepoint
          \t                     # separator
          (k[a-zA-Z0-9_]+)       # [2]field key
          \t                     # separator
          (.*)                   # [3]field value
        ",
    ).unwrap();
}

pub trait DataEntry {
    fn new(character: char) -> Self;
    fn update<'a>(&mut self, key: &'a str, value: &'a str);
}

pub fn parse_entries_from_str<T>(str: &str) -> Vec<T>
where
    T: DataEntry + Clone
{
    let mut entry_map: BTreeMap<char, T> = BTreeMap::default();

    for capture in UNIHAN_DATA_ENTRY_REGEX.captures_iter(str) {
        let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
        let chr = char::from_u32(code_point).unwrap();

        let key = &capture[2];
        let value = &capture[3];

        match entry_map.get(&chr) {
            None => {
                let mut entry = T::new(chr);
                entry.update(key, value);
                entry_map.insert(chr, entry);
            },
            Some(_) => {
                let mut entry = entry_map.get_mut(&chr).unwrap();
                entry.update(key, value);
            },
        }
    }

    entry_map.values().cloned().collect::<Vec<T>>()
}
