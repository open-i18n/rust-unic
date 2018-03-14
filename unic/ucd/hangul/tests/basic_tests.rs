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
use unic_ucd_hangul::{is_syllable, compose_syllable, decompose_syllable};

#[test]
fn test_is_syllable() {
    assert!(is_syllable('갏'));
    assert!(!is_syllable('A'));
    assert!(!is_syllable('韩'));
}

#[test]
fn test_decompose() {
    let mut decomposed_lv = vec![];
    {
        let mut collect_decomposed = |chr| {
            decomposed_lv.push(chr);
        };
        decompose_syllable('쮀', &mut collect_decomposed);
    }
    assert_eq!(decomposed_lv, ['ᄍ', 'ᅰ']);

    let mut decomposed_lvt = vec![];
    {
        let mut collect_decomposed = |chr| {
            decomposed_lvt.push(chr);
        };
        decompose_syllable('퓛', &mut collect_decomposed);
    }
    assert_eq!(decomposed_lvt, ['ᄑ', 'ᅱ', 'ᆶ']);
}

#[test]
fn test_compose() {
    let l = 'ᄑ';
    let v = 'ᅱ';
    let t = 'ᆶ';

    let lv = compose_syllable(l, v).unwrap();
    assert_eq!(lv, '퓌');

    let lvt = compose_syllable(lv, t).unwrap();
    assert_eq!(lvt, '퓛');
}
