#[macro_use]
extern crate unic_char_range;
extern crate unic_utils;
use unic_utils::CharDataTable;

#[test]
fn range_value_table() {
    const TABLE: CharDataTable<u32> = CharDataTable::Range(&[
        (chars!('a'..='g'), 1),
        (chars!('j'..='q'), 2),
        (chars!('w'..='z'), 3),
    ]);
    for ch in chars!('a'..='g') {
        assert_eq!(TABLE.find(ch), Some(1));
        assert_eq!(TABLE.find_defaulting(ch), 1);
    }
    for ch in chars!('h'..='i') {
        assert_eq!(TABLE.find(ch), None);
        assert_eq!(TABLE.find_defaulting(ch), 0);
    }
    for ch in chars!('j'..='q') {
        assert_eq!(TABLE.find(ch), Some(2));
        assert_eq!(TABLE.find_defaulting(ch), 2);
    }
    for ch in chars!('r'..='v') {
        assert_eq!(TABLE.find(ch), None);
        assert_eq!(TABLE.find_defaulting(ch), 0);
    }
    for ch in chars!('x'..='z') {
        assert_eq!(TABLE.find(ch), Some(3));
        assert_eq!(TABLE.find_defaulting(ch), 3);
    }
}
