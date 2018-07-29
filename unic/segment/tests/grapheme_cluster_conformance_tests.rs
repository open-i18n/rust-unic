// Copyright 2012-2015 The Rust Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_segment;

use unic_segment::Graphemes;

type TestData = &'static [(
    &'static str,
    &'static [&'static str],
    Option<&'static [&'static str]>,
)];

const TEST_DATA: TestData = include!("tables/grapheme_cluster_break_test_data.rsv");

/// Extra cases that the official test suite doesn't cover.
const EXTRA_TEST_DATA: TestData = include!("extra_grapheme_cluster_break_test_data.rsv");

#[test]
fn test_graphemes_conformance() {
    let tests = TEST_DATA.iter().chain(EXTRA_TEST_DATA);
    for &(input, graphemes, legacy_graphemes) in tests {
        macro_rules! assert_ {
            ($test:expr, $exp:expr, $name:expr) => {
                // collect into vector for better diagnostics in failure case
                let testing = $test.collect::<Vec<_>>();
                let expected = $exp.collect::<Vec<_>>();
                assert_eq!(
                    testing, expected,
                    "{} test for testcase ({:?}, {:?}, legacy: {:?}) failed.",
                    $name, input, graphemes, legacy_graphemes
                )
            };
        }

        let legacy_graphemes = match legacy_graphemes {
            Some(s) => s,
            None => graphemes,
        };

        // test forward iterator
        assert_!(
            Graphemes::new(input),
            graphemes.iter().cloned(),
            "Forward grapheme boundaries"
        );
        assert_!(
            Graphemes::new_legacy(input),
            legacy_graphemes.iter().cloned(),
            "Forward legacy grapheme boundaries"
        );

        // test reverse iterator
        assert_!(
            Graphemes::new(input).rev(),
            graphemes.iter().rev().cloned(),
            "Reverse grapheme boundaries"
        );
        assert_!(
            Graphemes::new_legacy(input).rev(),
            legacy_graphemes.iter().rev().cloned(),
            "Reverse legacy grapheme boundaries"
        );
    }
}
