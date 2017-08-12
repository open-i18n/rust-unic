#![feature(test)]

extern crate test;
extern crate unic_char_range;

use unic_char_range::*;

#[bench]
fn count(b: &mut test::Bencher) {
    b.iter(|| CharRange::all().iter().count())
}

#[bench]
// iterate the same range without skipping surrogates
fn count_baseline(b: &mut test::Bencher) {
    b.iter(|| (0..0x110000).take_while(|_| true).count())
}
