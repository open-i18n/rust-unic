// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod age;
mod bidi;
mod blocks;
mod case;
mod category;
mod common;
mod core;
mod hangul;
mod ident;
mod name;
mod normal;
mod segment;
mod segment_tests;

use writer::utils::clean_dir;

pub fn generate() {
    age::generate(&clean_dir("unic/ucd/age/tables"));
    bidi::generate(&clean_dir("unic/ucd/bidi/tables"));
    blocks::generate(&clean_dir("unic/ucd/block/tables"));
    case::generate(&clean_dir("unic/ucd/case/tables"));
    category::generate(&clean_dir("unic/ucd/category/tables"));
    common::generate(&clean_dir("unic/ucd/common/tables"));
    core::generate(&clean_dir("unic/ucd/version/tables"));
    hangul::generate(&clean_dir("unic/ucd/hangul/tables"));
    ident::generate(&clean_dir("unic/ucd/ident/tables"));
    name::generate(&clean_dir("unic/ucd/name/tables"));
    normal::generate(&clean_dir("unic/ucd/normal/tables"));
    segment::generate(&clean_dir("unic/ucd/segment/tables"));
    segment_tests::generate(&clean_dir("unic/ucd/segment/tests/tables"));
}
