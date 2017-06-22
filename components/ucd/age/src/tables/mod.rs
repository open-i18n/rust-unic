// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::cmp::Ordering::{Equal, Less, Greater};


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: (u64, u64, u64) = include!("unicode_version.rsv");

/// Represents values of the Unicode character property
/// [`Age`](http://www.unicode.org/reports/tr44/#Age).
///
/// The Age property indicates the first version in which a particular Unicode character was
/// assigned.
///
/// NOTE: `Age` is a **Catalog** type of property, meaning that it expands with newer version of
/// Unicode. In fact, this specific `enum` is expected to expand on every major or minor update of
/// Unicode.
///
/// * <http://www.unicode.org/reports/tr44/#Character_Age>
#[allow(non_camel_case_types, missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
// NOTE: [UNIC_UPDATE_ON_UNICODE_UPDATE] Source: `age_type.rsv`
pub enum Age {
    Unassigned,
    V10_0,
    V1_1,
    V2_0,
    V2_1,
    V3_0,
    V3_1,
    V3_2,
    V4_0,
    V4_1,
    V5_0,
    V5_1,
    V5_2,
    V6_0,
    V6_1,
    V6_2,
    V6_3,
    V7_0,
    V8_0,
    V9_0,
}

use self::Age::*;

pub const AGE_TABLE: &'static [(char, char, Age)] = include!("age_values.rsv");

impl Age {
    /// Find the character Age
    pub fn of(ch: char) -> Age {
        bsearch_range_value_table(ch, AGE_TABLE)
    }
}

// TODO: Generic'ize and move to `unic-ucd-utils`
// TODO: Optimize: put Unassigned ranges into the table, then only store (start, age) instead of
// (start, end, age)
fn bsearch_range_value_table(ch: char, r: &'static [(char, char, Age)]) -> Age {
    match r.binary_search_by(|&(lo, hi, _)| if lo <= ch && ch <= hi {
        Equal
    } else if hi < ch {
        Less
    } else {
        Greater
    }) {
        Ok(idx) => {
            let (_, _, cat) = r[idx];
            cat
        }
        Err(_) => Unassigned,
    }
}
