// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::Path;

use source::ucd::test::grapheme_break_test::{GraphemeBreakTest, GRAPHEME_BREAK_TESTS};
use source::ucd::test::word_break_test::{WordBreakTest, WORD_BREAK_TESTS};

use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_grapheme_cluster_break_test_data(dir);
    emit_word_break_test_data(dir);
}

fn str_escape(s: &str) -> String {
    format!(
        "\"{}\"",
        s.chars()
            .map(|c| c.escape_unicode().collect::<String>())
            .collect::<String>()
    )
}

fn chars_escape(cs: &[char]) -> String {
    str_escape(&cs.iter().collect::<String>())
}

fn legacy_should_break(rule: &String) -> bool {
    const EXTENDED_ONLY_RULES: &[&str] = &["9.1", "9.2"];
    EXTENDED_ONLY_RULES.contains(&rule.as_str())
}

fn emit_grapheme_cluster_break_test_data(dir: &Path) {
    let mut contents = "&[\n".to_owned();

    for case in GRAPHEME_BREAK_TESTS.entries.iter() {
        let GraphemeBreakTest {
            ref chars,
            ref breaks,
            ref rules,
            ..
        } = *case;

        contents.push_str("    (");

        // Source
        contents.push_str(&format!("{}, ", chars_escape(&chars)));

        // Extended Grapheme Clusters
        {
            contents.push_str("&[");
            let mut cluster: Vec<char> = vec![chars[0]];
            for (i, &brk) in breaks.iter().enumerate() {
                if brk {
                    contents.push_str(&format!("{}, ", chars_escape(&cluster)));
                    cluster.truncate(0);
                }
                cluster.push(chars[i + 1]);
            }
            contents.push_str(&format!("{}], ", chars_escape(&cluster)));
        }

        // Legacy Grapheme Clusters
        if rules.iter().any(legacy_should_break) {
            contents.push_str("Some(&[");
            let mut cluster: Vec<char> = vec![chars[0]];
            for (i, &brk) in breaks.iter().enumerate() {
                if brk || legacy_should_break(&rules[i]) {
                    contents.push_str(&format!("{}, ", chars_escape(&cluster)));
                    cluster.truncate(0);
                }
                cluster.push(chars[i + 1]);
            }
            contents.push_str(&format!("{}])", chars_escape(&cluster)));
        } else {
            contents.push_str("None");
        }

        contents.push_str("),\n");
    }

    contents.push_str("]");

    write(dir, "grapheme_cluster_break_test_data.rsv", &contents);
}

fn emit_word_break_test_data(dir: &Path) {
    let mut contents = "&[\n".to_owned();

    for case in WORD_BREAK_TESTS.entries.iter() {
        let WordBreakTest {
            ref chars,
            ref breaks,
            ..
        } = *case;

        contents.push_str("    (");

        // Source
        contents.push_str(&format!("{}, ", chars_escape(&chars)));

        // Words
        {
            contents.push_str("&[");
            let mut cluster: Vec<char> = vec![chars[0]];
            for (i, &brk) in breaks.iter().enumerate() {
                if brk {
                    contents.push_str(&format!("{}, ", chars_escape(&cluster)));
                    cluster.truncate(0);
                }
                cluster.push(chars[i + 1]);
            }
            contents.push_str(&format!("{}]", chars_escape(&cluster)));
        }

        contents.push_str("),\n");
    }

    contents.push_str("]");

    write(dir, "word_break_test_data.rsv", &contents);
}
