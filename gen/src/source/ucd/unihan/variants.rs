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
use std::str::FromStr;

use regex::Regex;

use source::utils::read;

use super::{DataEntry, parse_entries_from_str};

lazy_static! {
    /// [Variants]: http://www.unicode.org/reports/tr38/#N10211
    pub static ref UNIHAN_VARIANTS_DATA: VariantsData = {
        read("data/ucd/Unihan/Unihan_Variants.txt").parse().unwrap()
    };

    pub static ref VALUE_REGEX: Regex = Regex::new(
        r"(?x) # extended regex syntax
          U\+(2?[[:xdigit:]]{4}) # [1]codepoint
          <?(                    # [2]additional data
              k[[:alnum:]]+(:[TBZFJ]+)?(,k[[:alnum:]]+(:[TBZFJ]+)?)*
          )?
        ",
    ).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VariantsDataEntry {
    pub character: char,
    pub semantic_variants: Option<Vec<char>>, // FIXME: handle additional data
    pub simplified_variant: Option<char>,
    pub specialized_semantic_variants: Option<Vec<char>>, // FIXME: handle additional data
    pub traditional_variant: Option<char>,
    pub z_variants: Option<Vec<char>>, // FIXME: handle additional data
}

impl VariantsDataEntry {
    pub fn parse_value<'a>(str: &'a str) -> char {
        let capture = VALUE_REGEX.captures(str).unwrap();
        let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
        char::from_u32(code_point).unwrap()
    }

    pub fn parse_values_with_additional_data<'a>(str: &'a str) -> Vec<char> {
        let mut chars = vec![];
        for capture in VALUE_REGEX.captures_iter(str) {
            let code_point = u32::from_str_radix(&capture[1], 16).unwrap();
            let chr = char::from_u32(code_point).unwrap();
            chars.push(chr);
        }
        chars
    }
}

impl DataEntry for VariantsDataEntry {
    fn new(character: char) -> VariantsDataEntry {
        VariantsDataEntry {
            character: character,
            semantic_variants: None,
            simplified_variant: None,
            specialized_semantic_variants: None,
            traditional_variant: None,
            z_variants: None,
        }
    }

    fn update<'a>(&mut self, key: &'a str, value: &'a str) {
        match key {
            "kSemanticVariant" =>
                self.semantic_variants =
                    Some(VariantsDataEntry::parse_values_with_additional_data(value)),
            "kSimplifiedVariant" =>
                self.simplified_variant = Some(VariantsDataEntry::parse_value(value)),
            "kSpecializedSemanticVariant" =>
                self.specialized_semantic_variants =
                    Some(VariantsDataEntry::parse_values_with_additional_data(value)),
            "kTraditionalVariant" =>
                self.traditional_variant = Some(VariantsDataEntry::parse_value(value)),
            "kZVariant" =>
                self.z_variants =
                    Some(VariantsDataEntry::parse_values_with_additional_data(value)),
            _ => {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VariantsData {
    pub entries: Box<[VariantsDataEntry]>,
}

impl FromStr for VariantsData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(VariantsData {
            entries: parse_entries_from_str(str).into_boxed_slice(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::VariantsDataEntry;

    #[test]
    fn value_parse() {
        let sample_value = "U+54A8";
        let chr = VariantsDataEntry::parse_value(sample_value);
        assert_eq!(chr, '\u{54A8}')
    }

    #[test]
    fn value_parse_with_additional_data() {
        let sample_value = "U+54A8<kMatthews:T,kMeyerWempe U+8AEE<kMatthews,kMeyerWempe";
        let chars = VariantsDataEntry::parse_values_with_additional_data(sample_value);
        assert_eq!(chars, vec!['\u{54A8}', '\u{8AEE}'])
    }
}
