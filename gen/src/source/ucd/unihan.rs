// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms

use std::char;
use std::collections::BTreeMap;
use std::str::FromStr;

use regex::Regex;

use source::utils::read;

lazy_static! {
    pub static ref UNIHAN_DATA: UnihanData = {
        let mut unihan_data_files = "".to_string();
        unihan_data_files += &read("data/ucd/Unihan/Unihan_DictionaryIndices.txt");
        unihan_data_files += &read("data/ucd/Unihan/Unihan_DictionaryLikeData.txt");
        unihan_data_files += &read("data/ucd/Unihan/Unihan_IRGSources.txt");
        unihan_data_files += &read("data/ucd/Unihan/Unihan_NumericValues.txt");
        unihan_data_files += &read("data/ucd/Unihan/Unihan_OtherMappings.txt");
        unihan_data_files += &read("data/ucd/Unihan/Unihan_RadicalStrokeCounts.txt");
        unihan_data_files += &read("data/ucd/Unihan/Unihan_Readings.txt");
        unihan_data_files += &read("data/ucd/Unihan/Unihan_Variants.txt");
        unihan_data_files.parse().unwrap()
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnihanDataEntry {
    pub character: char,

    pub accounting_numeric: Option<u64>,
    pub other_numeric: Option<u64>,
    pub primary_numeric: Option<u64>,
}

impl UnihanDataEntry {
    pub fn new(character: char) -> UnihanDataEntry {
        UnihanDataEntry {
            character: character,
            accounting_numeric: None,
            other_numeric: None,
            primary_numeric: None,
        }
    }

    pub fn update<'a>(&mut self, key: &'a str, value: &'a str) {
        match key {
            "kAccountingNumeric" => self.accounting_numeric = value.parse::<u64>().ok(),
            "kOtherNumeric" => self.other_numeric = value.parse::<u64>().ok(),
            "kPrimaryNumeric" => self.primary_numeric = value.parse::<u64>().ok(),
            _ => println!("Key {} is not handled", key),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnihanData {
   pub entries: Box<[UnihanDataEntry]>,
}

impl FromStr for UnihanData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?xm)^ # every line
                  U\+([[:xdigit:]]{4,6}) # [1]codepoint
                  \t                     # separator
                  (k[a-zA-Z0-9_]+)       # [2]field key
                  \t                     # separator
                  (.*)                   # [3]field value
                ",
            ).unwrap();
        }

        let mut entry_map: BTreeMap<char, UnihanDataEntry> = BTreeMap::new();

        for capture in REGEX.captures_iter(str) {
            let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
            let chr = char::from_u32(code_point).unwrap();

            let key = &capture[2];
            let value = &capture[3];

            match entry_map.get(&chr) {
                None => {
                    let mut entry = UnihanDataEntry::new(chr);
                    entry.update(key, value);
                    entry_map.insert(chr, entry);
                },
                Some(_) => {
                    let mut entry = entry_map.get_mut(&chr).unwrap();
                    entry.update(key, value);
                },
            }
        }

        Ok(UnihanData {
            entries: entry_map.values()
                              .cloned()
                              .collect::<Vec<UnihanDataEntry>>()
                              .into_boxed_slice(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{UnihanData, UnihanDataEntry};

    #[test]
    fn unihan_data_entry_parse() {
        let mut entry1 = UnihanDataEntry::new('\u{3405}');
        entry1.other_numeric = Some(5);

        let mut entry2 = UnihanDataEntry::new('\u{4EDF}');
        entry2.accounting_numeric = Some(1000);

        let mut entry3 = UnihanDataEntry::new('\u{5146}');
        entry3.primary_numeric = Some(1000000000000);

        let entries = vec![
            entry1,
            entry2,
            entry3
        ];

        assert_eq!(
            "U+3405	kOtherNumeric	5\n\
             U+4EDF	kAccountingNumeric	1000\n\
             U+5146	kPrimaryNumeric	1000000000000\n\
             ".parse(),
            Ok(UnihanData { entries: entries.into_boxed_slice() })
        );
    }
}
