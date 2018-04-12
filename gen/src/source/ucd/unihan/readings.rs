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
    /// [Readings]: http://www.unicode.org/reports/tr38/#N1019C
    pub static ref UNIHAN_READINGS_DATA: ReadingsData = {
        read("data/ucd/Unihan/Unihan_Readings.txt").parse().unwrap()
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReadingsDataEntry {
    pub character: char,
    pub cantonese: Option<String>,
    pub definition: Option<String>,
    pub hangul: Option<String>,
    pub hanyu_pinlu: Option<String>,
    pub hanyu_pinyin: Option<String>,
    pub japanese_kun: Option<String>,
    pub japanese_on: Option<String>,
    pub korean: Option<String>,
    pub mandarin: Option<String>,
    pub tang: Option<String>,
    pub vietnamese: Option<String>,
    pub xhc_1983: Option<String>,
}

impl DataEntry for ReadingsDataEntry {
    fn new(character: char) -> ReadingsDataEntry {
        ReadingsDataEntry {
            character: character,
            cantonese: None,
            definition: None,
            hangul: None,
            hanyu_pinlu: None,
            hanyu_pinyin: None,
            japanese_kun: None,
            japanese_on: None,
            korean: None,
            mandarin: None,
            tang: None,
            vietnamese: None,
            xhc_1983: None,
        }
    }

    fn update<'a>(&mut self, key: &'a str, value: &'a str) {
        match key {
            "kCantonese" => self.cantonese = Some(value.to_owned()),
            "kDefinition" => self.definition = Some(value.to_owned()),
            "kHangul" => self.hangul = Some(value.to_owned()),
            "kHanyuPinlu" => self.hanyu_pinlu = Some(value.to_owned()),
            "kHanyuPinyin" => self.hanyu_pinyin = Some(value.to_owned()),
            "kJapaneseKun" => self.japanese_kun = Some(value.to_owned()),
            "kJapaneseOn" => self.japanese_on = Some(value.to_owned()),
            "kKorean" => self.korean = Some(value.to_owned()),
            "kMandarin" => self.mandarin = Some(value.to_owned()),
            "kTang" => self.tang = Some(value.to_owned()),
            "kVietnamese" => self.vietnamese = Some(value.to_owned()),
            "kXHC1983" => self.xhc_1983 = Some(value.to_owned()),
            _ => {},
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReadingsData {
    pub entries: Box<[ReadingsDataEntry]>,
}

impl FromStr for ReadingsData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        Ok(ReadingsData {
            entries: parse_entries_from_str(str).into_boxed_slice(),
        })
    }
}
