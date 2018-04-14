// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_ucd_unihan;

use unic_ucd_unihan::{definition_of, mandarin_of};

#[test]
fn test_definition() {
    assert_eq!(definition_of('\u{0001}'), None);
    assert_eq!(definition_of('\u{340c}'), Some("a tribe of savages in South China"));
}

#[test]
fn test_mandarin() {
    assert_eq!(mandarin_of('\u{0001}'), None);
    assert_eq!(mandarin_of('\u{340c}'), Some("yí"));
}
