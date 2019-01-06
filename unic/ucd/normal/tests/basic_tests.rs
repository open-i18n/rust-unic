// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use unic_ucd_normal;

use self::unic_ucd_normal::{CanonicalCombiningClass as CCC, DecompositionType as DT};

#[test]
fn test_canonical_combining_class_display() {
    assert_eq!(format!("{}", CCC::of('\u{0000}')), "0");
    assert_eq!(format!("{}", CCC::of('\u{0300}')), "230");
}

#[test]
fn test_decomposition_type_display() {
    assert_eq!(format!("{}", DT::of('\u{a0}').unwrap()), "No-Break");
}
