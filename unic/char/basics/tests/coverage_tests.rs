// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(
    bad_style,
    missing_debug_implementations,
    missing_docs,
    unconditional_recursion,
    unsafe_code
)]

extern crate unic_char_basics;
extern crate unic_char_range;

use unic_char_basics::{is_noncharacter, is_private_use};
use unic_char_range::CharRange;

/// Verify no code-point is both *Noncharacter* and *Private-Use*.
#[test]
fn test_overlap() {
    for codepoint in CharRange::all() {
        assert!(!(is_noncharacter(codepoint) && is_private_use(codepoint)));
    }
}
