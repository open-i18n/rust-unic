// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


extern crate unic_ucd_category;
extern crate unic_ucd_normal;
extern crate unic_utils;


use self::unic_ucd_category::GeneralCategory as GC;
use self::unic_ucd_normal::is_combining_mark;
use self::unic_utils::iter_all_chars;


/// `normal::is_combining_mark` and `GeneralCategory::is_mark()` are expected to return the same
/// results.
///
/// Since `unic-ucd-category` feature is not enabled, `is_combining_mark` resolves to the local
/// implementation.
#[test]
fn test_gen_cat_against_normal() {
    for cp in iter_all_chars() {
        assert_eq!(GC::of(cp).is_mark(), is_combining_mark(cp));
    }
}

