// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[test]
fn test_bidi_class_display() {
    use unic_ucd_bidi::bidi_class::abbr_names::*;

    assert_eq!(format!("{}", L), "Left-to-Right");
    assert_eq!(format!("{}", R), "Right-to-Left");
    assert_eq!(format!("{}", AL), "Right-to-Left Arabic");
    assert_eq!(format!("{}", FSI), "First Strong Isolate");
}
