extern crate unic_utils;
use unic_utils::CharDataTable;

use std::char;

#[test]
fn range_value_table() {
    const TABLE: &'static [(char, char, u32)] = &[('a', 'g', 1), ('j', 'q', 2), ('w', 'z', 3)];
    const DEFAULT: &u32 = &0;
    for i in ('a' as u32)..('g' as u32 + 1) {
        if let Some(needle) = char::from_u32(i) {
            assert_eq!(TABLE.find(needle), Some(&1));
            assert_eq!(TABLE.find_or(needle, DEFAULT), &1);
            assert_eq!(TABLE.find_or_else(needle, || DEFAULT), &1);
        }
    }
    for i in ('h' as u32)..('i' as u32 + 1) {
        if let Some(needle) = char::from_u32(i) {
            assert_eq!(TABLE.find(needle), None);
            assert_eq!(TABLE.find_or(needle, DEFAULT), DEFAULT);
            assert_eq!(TABLE.find_or_else(needle, || DEFAULT), DEFAULT);
        }
    }
    for i in ('j' as u32)..('q' as u32 + 1) {
        if let Some(needle) = char::from_u32(i) {
            assert_eq!(TABLE.find(needle), Some(&2));
            assert_eq!(TABLE.find_or(needle, DEFAULT), &2);
            assert_eq!(TABLE.find_or_else(needle, || DEFAULT), &2);
        }
    }
    for i in ('r' as u32)..('v' as u32 + 1) {
        if let Some(needle) = char::from_u32(i) {
            assert_eq!(TABLE.find(needle), None);
            assert_eq!(TABLE.find_or(needle, DEFAULT), DEFAULT);
            assert_eq!(TABLE.find_or_else(needle, || DEFAULT), DEFAULT);
        }
    }
    for i in ('x' as u32)..('z' as u32 + 1) {
        if let Some(needle) = char::from_u32(i) {
            assert_eq!(TABLE.find(needle), Some(&3));
            assert_eq!(TABLE.find_or(needle, DEFAULT), &3);
            assert_eq!(TABLE.find_or_else(needle, || DEFAULT), &3);
        }
    }
}
