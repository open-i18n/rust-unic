// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::char;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;

use reader::ucd::unicode_data::{UnicodeDataEntry, UNICODE_DATA};

use writer::utils::tables::ToRangeCharTable;
use writer::ucd::unicode_version;
use writer::utils::write;


#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct CategoryData<'a>(BTreeMap<char, &'a str>);

impl<'a> CategoryData<'a> {
    fn emit(&self, dir: &Path) {
        let CategoryData(ref map) = *self;
        let contents = map.to_range_char_table(Display::fmt);
        write(&dir, "general_category.rsv", &contents);
    }
}

impl<'a, I> From<I> for CategoryData<'a>
where
    I: Iterator<Item = &'a UnicodeDataEntry>,
{
    fn from(it: I) -> Self {
        let mut map = BTreeMap::<char, &str>::new();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        for &UnicodeDataEntry { character, ref general_category, .. } in it {
            map.insert(character, general_category);
        }

        CategoryData(map)
    }
}

pub fn generate(dir: &Path) {
    unicode_version::emit(&dir);
    CategoryData::from(UNICODE_DATA.0.iter()).emit(dir);
}
