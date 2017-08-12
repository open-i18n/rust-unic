extern crate unic_char_range;

use std::{char, u32};
use unic_char_range::*;

fn all_chars() -> Vec<char> {
    (u32::MIN..u32::MAX)
        .take_while(|&u| u <= char::MAX as u32)
        .filter_map(char::from_u32)
        .collect()
}

#[test]
fn iter_all_chars() {
    assert!(CharRange::all().iter().eq(all_chars()));
}

#[test]
fn iter_all_chars_rev() {
    assert!(CharRange::all().iter().rev().eq(all_chars().into_iter().rev()))
}

#[test]
fn iter_all_chars_mixed_next_back() {
    let mut custom = CharRange::all().iter();
    let mut simple = all_chars().into_iter();
    while let Some(custom_char) = custom.next() {
        assert_eq!(Some(custom_char), simple.next());
        assert_eq!(custom.next_back(), simple.next_back());
    }
    assert_eq!(None, simple.next());
}

#[test]
fn iter_fused() {
    let mut iter = CharRange::all().iter();
    let mut fused = all_chars().into_iter().fuse();
    assert!(iter.by_ref().eq(fused.by_ref()));
    for _ in 0..100 {
        assert_eq!(iter.next(), fused.next());
    }
}

#[test]
fn iter_exact_trusted_len() {
    fn assert_presents_right_len<I: ExactSizeIterator>(iter: &I, len: usize) {
        assert_eq!(iter.len(), len);
        assert_eq!(iter.size_hint(), (len, Some(len)));
    }

    let mut iter = CharRange::all().iter();
    let mut predicted_length = iter.len();

    while let Some(_) = iter.next() {
        predicted_length -= 1;
        assert_presents_right_len(&iter, predicted_length);
    }
}
