// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub mod test;

pub mod blocks;
pub mod derived_age;
pub mod derived_core_properties;
pub mod derived_normalization_props;
pub mod grapheme_break_property;
pub mod jamo;
pub mod prop_list;
pub mod readme;
pub mod sentence_break_property;
pub mod unicode_data;
pub mod word_break_property;

use regex::Regex;

lazy_static! {
    pub static ref BINARY_PROPERTIES_REGEX: Regex = Regex::new(
        r"(?xm)^
          ([[:xdigit:]]{4,6})        # low
          (?:..([[:xdigit:]]{4,6}))? # high
          \s+;\s+
          (\w+)                      # property
         ",
    ).unwrap();
}
