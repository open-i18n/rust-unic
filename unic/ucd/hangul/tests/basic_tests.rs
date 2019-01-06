// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use unic_ucd_hangul::{compose_syllable, decompose_syllable, is_syllable};

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

#[test]
fn test_decompose_then_compose() {
    let sample_hangul = '갓';
    assert!(is_syllable(sample_hangul));

    let mut decomposed = vec![];
    {
        let mut collect_decomposed = |chr| {
            decomposed.push(chr);
        };
        decompose_syllable(sample_hangul, &mut collect_decomposed);
    }
    assert_eq!(decomposed, ['ᄀ', 'ᅡ', 'ᆺ']);

    let composed_lv = compose_syllable(decomposed[0], decomposed[1]).unwrap();
    let composed = compose_syllable(composed_lv, decomposed[2]).unwrap();
    assert_eq!(composed, sample_hangul);
}
