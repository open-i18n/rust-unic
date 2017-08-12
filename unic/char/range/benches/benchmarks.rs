#![feature(test)]

extern crate test;
extern crate unic_char_range;

use std::char;
use unic_char_range::*;

#[inline(never)]
fn yes_man<T>(_: &T) -> bool {
    true
}

#[bench]
fn count(b: &mut test::Bencher) {
    b.iter(|| CharRange::all().iter().take_while(yes_man).count())
}

#[bench]
// iterate the same range without skipping surrogates
fn count_baseline(b: &mut test::Bencher) {

    b.iter(|| (0..(char::MAX as u32 + 1)).take_while(yes_man).count())
}
