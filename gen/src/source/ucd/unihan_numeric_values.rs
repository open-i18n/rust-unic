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

use source::utils::read;

use super::UNIHAN_DATA_ENTRY_REGEX;

lazy_static! {
    /// [Numeric values]: http://www.unicode.org/reports/tr38/#N1024D
    pub static ref UNIHAN_NUMERIC_VALUES_DATA: UnihanNumericValuesData = {
        read("data/ucd/Unihan/Unihan_NumericValues.txt").parse().unwrap()
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnihanNumericValuesDataEntry {
    pub character: char,
    pub accounting_numeric: Option<u64>,
    pub other_numeric: Option<u64>,
    pub primary_numeric: Option<u64>,
}

impl UnihanNumericValuesDataEntry {
    pub fn new(character: char) -> UnihanNumericValuesDataEntry {
        UnihanNumericValuesDataEntry {
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
            _ => {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnihanNumericValuesData {
   pub entries: Box<[UnihanNumericValuesDataEntry]>,
}

impl FromStr for UnihanNumericValuesData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut entry_map: BTreeMap<char, UnihanNumericValuesDataEntry> = BTreeMap::default();

        for capture in UNIHAN_DATA_ENTRY_REGEX.captures_iter(str) {
            let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
            let chr = char::from_u32(code_point).unwrap();

            let key = &capture[2];
            let value = &capture[3];

            match entry_map.get(&chr) {
                None => {
                    let mut entry = UnihanNumericValuesDataEntry::new(chr);
                    entry.update(key, value);
                    entry_map.insert(chr, entry);
                },
                Some(_) => {
                    let mut entry = entry_map.get_mut(&chr).unwrap();
                    entry.update(key, value);
                },
            }
        }

        Ok(UnihanNumericValuesData {
            entries: entry_map.values()
                              .cloned()
                              .collect::<Vec<UnihanNumericValuesDataEntry>>()
                              .into_boxed_slice(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{UnihanNumericValuesData, UnihanNumericValuesDataEntry};

    #[test]
    fn unihan_numeric_values_data_entry_parse() {
        let mut entry1 = UnihanNumericValuesDataEntry::new('\u{3405}');
        entry1.other_numeric = Some(5);

        let mut entry2 = UnihanNumericValuesDataEntry::new('\u{4EDF}');
        entry2.accounting_numeric = Some(1000);

        let mut entry3 = UnihanNumericValuesDataEntry::new('\u{5146}');
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
            Ok(UnihanNumericValuesData { entries: entries.into_boxed_slice() })
        );
    }
}
