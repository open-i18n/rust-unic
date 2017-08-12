#![feature(test)]

extern crate test;
extern crate unic_char_range;

use std::char;
use unic_char_range::CharRange;

#[bench]
fn forward_iteration(b: &mut test::Bencher) {
    b.iter(|| CharRange::all().count())
}

#[bench]
fn forward_iteration_baseline(b: &mut test::Bencher) {
    b.iter(|| (0..0x110000).filter_map(char::from_u32).count())
}

#[bench]
fn reverse_iteration(b: &mut test::Bencher) {
    b.iter(|| CharRange::all().rev().count())
}

#[bench]
fn reverse_iteration_baseline(b: &mut test::Bencher) {
    b.iter(|| (0..0x110000).rev().filter_map(char::from_u32).count())
}
