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

use crate::source::emoji::emoji_data::EMOJI_DATA;
use crate::source::ucd::grapheme_break_property::GRAPHEME_CLUSTER_BREAK_DATA;
use crate::source::ucd::test::grapheme_break_test::{GraphemeBreakTest, GRAPHEME_BREAK_TESTS};
use crate::source::ucd::test::word_break_test::{WordBreakTest, WORD_BREAK_TESTS};
use crate::source::ucd::word_break_property::WORD_BREAK_DATA;

use crate::writer::utils::tables::ToDirectCharTable;
use crate::writer::utils::write;

pub fn generate(dir: &Path) {
    emit_grapheme_cluster_break_test_data(dir);
    emit_word_break_test_data(dir);
}

/// Names of the auxiliary sets that the test files combine with the break property values to form
/// partition classes (`X_Y` for the intersection of `X` and `Y`, `XmY` for `X` minus `Y`).
const PARTITION_SETS: &[&str] = &[
    "ConjunctExtender",
    "ConjunctLinker",
    "LinkingConsonant",
    "ExtPict",
    "ExtCccZwj",
    "Extend",
    "ALetter",
];

/// Strips the partition-class suffixes (e.g. `Extend_ConjunctExtender`,
/// `XXmConjunctLinkermLinkingConsonantmExtPict`) from a label used in the test files' comments,
/// leaving the break property value (or auxiliary set) name it is based on.
fn base_class(label: &str) -> &str {
    let mut s = label;
    'outer: loop {
        for set in PARTITION_SETS {
            for sep in ['m', '_'] {
                let suffix = format!("{}{}", sep, set);
                if s.len() > suffix.len() && s.ends_with(&suffix) {
                    s = &s[..s.len() - suffix.len()];
                    continue 'outer;
                }
            }
        }
        return s;
    }
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
            let gcb = match base_class(&char_gcbs[i]) {
                // These are InCB-based sets; their members have no GCB value of their own.
                "XX" | "ConjunctLinker" | "LinkingConsonant" => "Other",
                c => c,
            };

            if EMOJI_DATA.extended_pictographic.contains(ch) {
                // If the character is defined as "Extended Pictographic" in the emoji data,
                // then treat it as an extended pictographic when generating the test table
                map.insert(*ch, "ExtPict");
            } else {
                // Sanity-check the label mapping against the property data itself.
                let expected = GRAPHEME_CLUSTER_BREAK_DATA
                    .map
                    .get(ch)
                    .map_or("Other", String::as_str);
                assert!(
                    gcb == expected || (gcb == "RI" && expected == "Regional_Indicator"),
                    "GraphemeBreakTest label {:?} for U+{:04X} maps to {:?}, property data says {:?}",
                    char_gcbs[i],
                    *ch as u32,
                    gcb,
                    expected
                );
                if map.contains_key(ch) {
                    assert_eq!(map[ch], gcb);
                } else {
                    map.insert(*ch, gcb);
                }
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
            // `Extended_Pictographic` is not a Word_Break value; it is only consulted by rule
            // WB3c, so its members are listed with their real Word_Break value.
            let wb = match base_class(&char_gcbs[i]) {
                "XX" | "ExtPict" => "Other",
                c => c,
            };

            // Sanity-check the label mapping against the property data itself.
            let expected = WORD_BREAK_DATA.map.get(ch).map_or("Other", String::as_str);
            assert!(
                wb == expected || (wb == "RI" && expected == "Regional_Indicator"),
                "WordBreakTest label {:?} for U+{:04X} maps to {:?}, property data says {:?}",
                char_gcbs[i],
                *ch as u32,
                wb,
                expected
            );
            if map.contains_key(ch) {
                assert_eq!(map[ch], wb);
            } else {
                map.insert(*ch, wb);
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
