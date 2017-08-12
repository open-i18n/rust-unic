#![feature(test)]

extern crate test;
extern crate unic_char_range;

use std::char;
use unic_char_range::CharRange;

#[bench]
fn count(b: &mut test::Bencher) {
    b.iter(|| CharRange::closed('\0', '\u{10FFFF}').count())
}

#[bench]
fn count_baseline(b: &mut test::Bencher) {
    b.iter(|| (0..0x110000).filter_map(char::from_u32).count())
}
