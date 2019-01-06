// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate unic_char_range;

use unic_ucd_category::GeneralCategory;
use unic_ucd_common::{is_alphabetic, is_alphanumeric, is_control, is_numeric, is_white_space};

macro_rules! assert_char {
    (
        $ch:expr,
        if $precondition:expr => [
            $( $condition:expr $(,)* ),+
        ]
    ) => (
        if $precondition {
            $(
                assert!(
                    $condition,
                    "Code-point U+{:04x} meets precondition `{}` but not condition `{}`.",
                    $ch as u32,
                    stringify!($precondition),
                    stringify!($condition)
                );
            )+
        }
    );
}

#[test]
fn test_values_internally_and_against_general_category() {
    for ch in chars!(..) {
        let ch_is_alphabetic = is_alphabetic(ch);
        let ch_is_alphanumeric = is_alphanumeric(ch);
        let ch_is_control = is_control(ch);
        let ch_is_numeric = is_numeric(ch);
        let ch_is_white_space = is_white_space(ch);

        let gc = GeneralCategory::of(ch);
        let gc_is_letter_number = gc == GeneralCategory::LetterNumber;
        let gc_is_control = gc == GeneralCategory::Control;

        // Alphabetic
        assert_char!(ch, if ch_is_alphabetic => [
            !ch_is_control,
            !ch_is_numeric || gc_is_letter_number,
            !ch_is_white_space,
            ch_is_alphanumeric,
        ]);

        // Control
        assert_char!(ch, if ch_is_control => [
            !ch_is_alphabetic,
            // Has overlap with ch_is_white_space, like U+0009..U+000D, U+0085
            !ch_is_numeric,
            !ch_is_alphanumeric,
        ]);

        // Numeric
        assert_char!(ch, if ch_is_numeric => [
            !ch_is_alphabetic || gc_is_letter_number,
            !ch_is_control,
            !ch_is_white_space,
            ch_is_alphanumeric,
        ]);

        // White Space
        assert_char!(ch, if ch_is_white_space => [
            !ch_is_alphabetic,
            // has overlap with ch_is_control, like U+0009..U+000D, U+0085
            !ch_is_numeric,
            !ch_is_alphanumeric,
        ]);

        // Alphanumeric
        assert_char!(ch, if ch_is_alphanumeric => [
            ch_is_alphabetic || ch_is_numeric,
            !ch_is_control,
            !ch_is_white_space,
        ]);

        // General Category vs common
        assert_char!(ch, if ch_is_control => [ gc_is_control ]);
        assert_char!(ch, if gc_is_control => [ ch_is_control ]);
        assert_char!(ch, if gc.is_letter() => [ ch_is_alphabetic ]);
        assert_char!(ch, if gc.is_number() => [ ch_is_numeric ]);
    }
}
