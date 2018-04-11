// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms

use std::str::FromStr;

use source::utils::read;

use super::{DataEntry, parse_entries_from_str};

lazy_static! {
    /// [Numeric values]: http://www.unicode.org/reports/tr38/#N1024D
    pub static ref UNIHAN_NUMERIC_VALUES_DATA: NumericValuesData = {
        read("data/ucd/Unihan/Unihan_NumericValues.txt").parse().unwrap()
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NumericValuesDataEntry {
    pub character: char,
    pub accounting_numeric: Option<u64>,
    pub other_numeric: Option<u64>,
    pub primary_numeric: Option<u64>,
}

impl DataEntry for NumericValuesDataEntry {
    fn new(character: char) -> NumericValuesDataEntry {
        NumericValuesDataEntry {
            character: character,
            accounting_numeric: None,
            other_numeric: None,
            primary_numeric: None,
        }
    }

    fn update<'a>(&mut self, key: &'a str, value: &'a str) {
        match key {
            "kAccountingNumeric" => self.accounting_numeric = value.parse::<u64>().ok(),
            "kOtherNumeric" => self.other_numeric = value.parse::<u64>().ok(),
            "kPrimaryNumeric" => self.primary_numeric = value.parse::<u64>().ok(),
            _ => {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NumericValuesData {
   pub entries: Box<[NumericValuesDataEntry]>,
}

impl FromStr for NumericValuesData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(NumericValuesData {
            entries: parse_entries_from_str(str).into_boxed_slice(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::DataEntry;
    use super::{NumericValuesData, NumericValuesDataEntry};

    #[test]
    fn data_entry_parse() {
        let mut entry1 = NumericValuesDataEntry::new('\u{3405}');
        entry1.other_numeric = Some(5);

        let mut entry2 = NumericValuesDataEntry::new('\u{4EDF}');
        entry2.accounting_numeric = Some(1000);

        let mut entry3 = NumericValuesDataEntry::new('\u{5146}');
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
            Ok(NumericValuesData {
                entries: entries.into_boxed_slice(),
            }),
        );
    }
}
