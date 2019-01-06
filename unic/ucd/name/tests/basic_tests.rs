// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use unic_ucd_name::Name;

#[test]
fn test_name_str() {
    // == NR1: Hangul Syllable ==

    assert_eq!(Name::of('곲').unwrap().to_string(), "HANGUL SYLLABLE GOBS");

    // == NR2: Ideographs ==

    assert_eq!(
        Name::of('言').unwrap().to_string(),
        "CJK UNIFIED IDEOGRAPH-8A00"
    );
    assert_eq!(
        Name::of('\u{17005}').unwrap().to_string(),
        "TANGUT IDEOGRAPH-17005"
    );
    assert_eq!(
        Name::of('\u{1B175}').unwrap().to_string(),
        "NUSHU CHARACTER-1B175"
    );
    assert_eq!(
        Name::of('理').unwrap().to_string(),
        "CJK COMPATIBILITY IDEOGRAPH-F9E4"
    );

    // == NR3: Explicitly Listed Names ==

    assert_eq!(Name::of('A').unwrap().to_string(), "LATIN CAPITAL LETTER A");

    // == NR4: Null Name ==

    assert_eq!(Name::of('\u{10FFFF}'), None);
}

#[test]
fn test_name_cmp() {
    macro_rules! name_cmp_assert {
        ($lhs: expr, $operator: tt, $rhs: expr) => {
            let lhs_name = Name::of($lhs);
            let rhs_name = Name::of($rhs);
            assert!(lhs_name $operator rhs_name);
            assert!(lhs_name.unwrap().to_string() $operator rhs_name.unwrap().to_string());
        }
    }

    // == NR1 ==

    name_cmp_assert!('곲', >, '가'); // GOBS > GA
    name_cmp_assert!('갆', <, '뉆'); // GANH < NWELM
    name_cmp_assert!('둉', <, '럋'); // DYONG < RYAC
    name_cmp_assert!('곲', >, '言');
    name_cmp_assert!('곲', >, '理');
    name_cmp_assert!('곲', <, '\u{17005}');
    name_cmp_assert!('곲', <, '\u{1B175}');
    name_cmp_assert!('곲', <, 'A');

    // == NR2 - CJK UNIFIED IDEOGRAPH ==

    name_cmp_assert!('言', <, '곲');
    name_cmp_assert!('言', >, '文');
    name_cmp_assert!('言', <, '\u{17005}');
    name_cmp_assert!('言', <, '\u{1B175}');
    name_cmp_assert!('言', >, '理');
    name_cmp_assert!('言', <, 'A');

    // == NR2 - TANGUT IDEOGRAPH ==

    name_cmp_assert!('\u{17005}', >, '곲');
    name_cmp_assert!('\u{17005}', >, '言');
    name_cmp_assert!('\u{17005}', <, '\u{170A0}');
    name_cmp_assert!('\u{17005}', >, '\u{1B175}');
    name_cmp_assert!('\u{17005}', >, '理');
    name_cmp_assert!('\u{17005}', >, 'A');

    // == NR2 - NUSHU CHARACTER ==

    name_cmp_assert!('\u{1B175}', >, '곲');
    name_cmp_assert!('\u{1B175}', >, '言');
    name_cmp_assert!('\u{1B175}', <, '\u{17005}');
    name_cmp_assert!('\u{1B175}', <, '\u{1B182}');
    name_cmp_assert!('\u{1B175}', >, '理');
    name_cmp_assert!('\u{1B175}', >, 'A');

    // == NR2 - CJK COMPATIBILITY IDEOGRAPH ==

    name_cmp_assert!('理', <, '곲');
    name_cmp_assert!('理', <, '言');
    name_cmp_assert!('理', <, '\u{17005}');
    name_cmp_assert!('理', <, '\u{1B175}');
    name_cmp_assert!('理', <, '臨');
    name_cmp_assert!('理', <, 'A');

    // == NR3 ==

    name_cmp_assert!('A', >, '곲');
    name_cmp_assert!('A', >, '言');
    name_cmp_assert!('A', <, '\u{17005}');
    name_cmp_assert!('A', <, '\u{1B175}');
    name_cmp_assert!('A', >, '理');
    name_cmp_assert!('A', <, 'B');
}
