// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_ucd_hangul;
use unic_ucd_hangul::hangul;

#[test]
fn test_is_syllable() {
    assert!(hangul::is_syllable('갏'));
    assert!(!hangul::is_syllable('A'));
    assert!(!hangul::is_syllable('韩'));
}

#[test]
fn test_decompose() {
    let mut decomposed_lv = vec![];
    {
        let mut collect_decomposed = |chr| {
            decomposed_lv.push(chr);
        };
        hangul::decompose('쮀', &mut collect_decomposed);
    }
    assert_eq!(decomposed_lv, ['ᄍ', 'ᅰ']);

    let mut decomposed_lvt = vec![];
    {
        let mut collect_decomposed = |chr| {
            decomposed_lvt.push(chr);
        };
        hangul::decompose('퓛', &mut collect_decomposed);
    }
    assert_eq!(decomposed_lvt, ['ᄑ', 'ᅱ', 'ᆶ']);
}

#[test]
fn test_compose() {
    let l = 'ᄑ';
    let v = 'ᅱ';
    let t = 'ᆶ';

    let lv = hangul::compose(l, v).unwrap();
    assert_eq!(lv, '퓌');

    let lvt = hangul::compose(lv, t).unwrap();
    assert_eq!(lvt, '퓛');
}
