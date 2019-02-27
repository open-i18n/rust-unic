// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::path::Path;

use crate::source::ucd::test::grapheme_break_test::{GraphemeBreakTest, GRAPHEME_BREAK_TESTS};
use crate::source::ucd::test::word_break_test::{WordBreakTest, WORD_BREAK_TESTS};

use crate::writer::utils::tables::ToDirectCharTable;
use crate::writer::utils::write;

pub fn generate(dir: &Path) {
    emit_grapheme_cluster_break_test_data(dir);
    emit_word_break_test_data(dir);
}

fn emit_grapheme_cluster_break_test_data(dir: &Path) {
    let mut map = BTreeMap::default();

    for case in GRAPHEME_BREAK_TESTS.entries.iter() {
        let GraphemeBreakTest {
            ref chars,
            ref char_gcbs,
            ..
        } = *case;

        for (i, ch) in chars.iter().enumerate() {
            let gcb = match char_gcbs[i].as_str() {
                "Extend_ExtCccZwj" => "Extend",
                "ZWJ_ExtCccZwj" => "ZWJ",
                c => c
            };
            if map.contains_key(ch) {
                assert_eq!(map[ch], gcb);
            } else {
                map.insert(*ch, gcb.clone());
            }
        }
    }

    write(
        dir,
        "grapheme_cluster_break_test_data.rsv",
        &map.to_direct_char_table(|val, f| write!(f, "{}", val)),
    );
}

fn emit_word_break_test_data(dir: &Path) {
    let mut map = BTreeMap::default();

    for case in WORD_BREAK_TESTS.entries.iter() {
        let WordBreakTest {
            ref chars,
            ref char_gcbs,
            ..
        } = *case;

        for (i, ch) in chars.iter().enumerate() {
            let ref gcb = char_gcbs[i];
            if map.contains_key(ch) {
                assert_eq!(map[ch], *gcb);
            } else {
                map.insert(*ch, gcb.clone());
            }
        }
    }

    write(
        dir,
        "word_break_test_data.rsv",
        &map.to_direct_char_table(|val, f| write!(f, "{}", val)),
    );
}

/* TODO
fn emit_sentence_break_test_data(dir: &Path) {
    let mut map = BTreeMap::default();

    for case in SENTENCE_BREAK_TESTS.entries.iter() {
        let SentenceBreakTest {
            ref chars,
            ref char_gcbs,
            ..
        } = *case;

        for (i, ch) in chars.iter().enumerate() {
            let ref gcb = char_gcbs[i];
            if map.contains_key(ch) {
                assert_eq!(map[ch], *gcb);
            } else {
                map.insert(*ch, gcb.clone());
            }
        }
    }

    write(
        dir,
        "sentence_break_test_data.rsv",
        &map.to_direct_char_table(|val, f| write!(f, "{}", val)),
    );
}
*/
