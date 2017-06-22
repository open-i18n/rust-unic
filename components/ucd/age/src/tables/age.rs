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


use std::cmp::Ordering;

use unic_ucd_core::UnicodeVersion;


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
pub enum Age {
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
    V10_0,
    // [UNIC_UPDATE_ON_UNICODE_UPDATE] Source: `age_type.rsv`
    Unassigned,
}

use Age::*;

pub const AGE_TABLE: &'static [(char, char, Age)] = include!("age_values.rsv");

impl Age {
    /// Find the character Age
    pub fn of(ch: char) -> Age {
        bsearch_range_value_table(ch, AGE_TABLE)
    }

    /// Get `UnicodeVersion` of an `Age` value, or `None`
    pub fn to_unicode_version(self) -> Option<UnicodeVersion> {
        // TODO: Auto-generate match expression
        match self {
            Unassigned => None,
            V1_1 => Some(UnicodeVersion(1, 1, 0)),
            V2_0 => Some(UnicodeVersion(2, 0, 0)),
            V2_1 => Some(UnicodeVersion(2, 1, 0)),
            V3_0 => Some(UnicodeVersion(3, 0, 0)),
            V3_1 => Some(UnicodeVersion(3, 1, 0)),
            V3_2 => Some(UnicodeVersion(3, 2, 0)),
            V4_0 => Some(UnicodeVersion(4, 0, 0)),
            V4_1 => Some(UnicodeVersion(4, 1, 0)),
            V5_0 => Some(UnicodeVersion(5, 0, 0)),
            V5_1 => Some(UnicodeVersion(5, 1, 0)),
            V5_2 => Some(UnicodeVersion(5, 2, 0)),
            V6_0 => Some(UnicodeVersion(6, 0, 0)),
            V6_1 => Some(UnicodeVersion(6, 1, 0)),
            V6_2 => Some(UnicodeVersion(6, 2, 0)),
            V6_3 => Some(UnicodeVersion(6, 3, 0)),
            V7_0 => Some(UnicodeVersion(7, 0, 0)),
            V8_0 => Some(UnicodeVersion(8, 0, 0)),
            V9_0 => Some(UnicodeVersion(9, 0, 0)),
            V10_0 => Some(UnicodeVersion(10, 0, 0)),
            // [UNIC_UPDATE_ON_UNICODE_UPDATE] Source: `Age`
        }
    }

    /// Get `Age` value from `UnicodeVersion`
    pub fn from_unicode_version(version: UnicodeVersion) -> Option<Age> {
        // TODO: Auto-generate match expression
        match version {
            UnicodeVersion(1, 1, 0) => Some(V1_1),
            UnicodeVersion(2, 0, 0) => Some(V2_0),
            UnicodeVersion(2, 1, 0) => Some(V2_1),
            UnicodeVersion(3, 0, 0) => Some(V3_0),
            UnicodeVersion(3, 1, 0) => Some(V3_1),
            UnicodeVersion(3, 2, 0) => Some(V3_2),
            UnicodeVersion(4, 0, 0) => Some(V4_0),
            UnicodeVersion(4, 1, 0) => Some(V4_1),
            UnicodeVersion(5, 0, 0) => Some(V5_0),
            UnicodeVersion(5, 1, 0) => Some(V5_1),
            UnicodeVersion(5, 2, 0) => Some(V5_2),
            UnicodeVersion(6, 0, 0) => Some(V6_0),
            UnicodeVersion(6, 1, 0) => Some(V6_1),
            UnicodeVersion(6, 2, 0) => Some(V6_2),
            UnicodeVersion(6, 3, 0) => Some(V6_3),
            UnicodeVersion(7, 0, 0) => Some(V7_0),
            UnicodeVersion(8, 0, 0) => Some(V8_0),
            UnicodeVersion(9, 0, 0) => Some(V9_0),
            UnicodeVersion(10, 0, 0) => Some(V10_0),
            // [UNIC_UPDATE_ON_UNICODE_UPDATE] Source: `Age`
            _ => None,
        }
    }
}

// TODO: Generic'ize and move to `unic-ucd-utils`
// TODO: Optimize: put Unassigned ranges into the table, then only store (start, age) instead of
// (start, end, age)
fn bsearch_range_value_table(ch: char, r: &'static [(char, char, Age)]) -> Age {
    match r.binary_search_by(|&(lo, hi, _)| if lo <= ch && ch <= hi {
        Ordering::Equal
    } else if hi < ch {
        Ordering::Less
    } else {
        Ordering::Greater
    }) {
        Ok(idx) => {
            let (_, _, cat) = r[idx];
            cat
        }
        Err(_) => Unassigned,
    }
}
