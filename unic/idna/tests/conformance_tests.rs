// Copyright 2013-2014 The rust-url developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_idna;

use std::char;

#[test]
fn test_idn_test_data() {
    // Source: https://www.unicode.org/Public/idna/latest/IdnaTestV2.txt
    for (line_idx, line) in include_str!("../../../data/idna/test/IdnaTestV2.txt")
        .lines()
        .enumerate()
    {
        if line == "" || line.starts_with('#') {
            continue;
        }

        // Remove comments
        let mut line = match line.find('#') {
            Some(index) => &line[0..index],
            None => line,
        };

        let mut pieces = line.split(';').map(|x| x.trim());

        let source = pieces.next().unwrap();
        let to_unicode = empty_or(&mut pieces, source);
        let to_unicode_status = empty_or(&mut pieces, "[]");
        let to_ascii_n = empty_or(&mut pieces, to_unicode);
        let to_ascii_n_status = empty_or(&mut pieces, to_unicode_status);
        let to_ascii_t = empty_or(&mut pieces, to_ascii_n);
        let to_ascii_t_status = empty_or(&mut pieces, to_ascii_n_status);

        assert_unicode(source, to_unicode, to_unicode_status, line_idx);
        assert_ascii(source, to_ascii_n, to_ascii_n_status, false, line_idx);
        assert_ascii(source, to_ascii_t, to_ascii_t_status, true, line_idx);
    }
}

fn empty_or<'a>(iter: &mut Iterator<Item = &'a str>, or: &'a str) -> &'a str {
    let next = iter.next().unwrap();

    if next.is_empty() {
        or
    } else {
        next
    }
}

fn assert_unicode(source: &str, expected: &str, status: &str, line_idx: usize)
{
    let (actual, result) = unic_idna::to_unicode(
        source,
        unic_idna::Flags {
            use_std3_ascii_rules: true,
            transitional_processing: false,
            verify_dns_length: true,
        },
    );

    if let Ok(()) = result {
        assert_eq!(
            actual,
            expected,
            "data/idna/test/IdnaTestV2.txt:{}",
            line_idx + 1
        );

        if status.starts_with("[C") {
            // http://unicode.org/reports/tr46/#Deviations
            // applications that perform IDNA2008 lookup are not required to check
            // for these contexts
            return;
        }

        if status == "[V2]" {
            // Everybody ignores V2
            // https://github.com/servo/rust-url/pull/240
            // https://github.com/whatwg/url/issues/53#issuecomment-181528158
            // https://www.unicode.org/review/pri317/
            return;
        }

        assert_eq!(
            status,
            "[]",
            "data/idna/test/IdnaTestV2.txt:{}",
            line_idx + 1
        );
    } else {
        assert_ne!(
            status,
            "[]",
            "data/idna/test/IdnaTestV2.txt:{}",
            line_idx + 1
        );
    }

}

fn assert_ascii(source: &str, expected: &str, status: &str, transitional: bool, line_idx: usize) {
    let result = unic_idna::to_ascii(
        source,
        unic_idna::Flags {
            use_std3_ascii_rules: true,
            transitional_processing: transitional,
            verify_dns_length: true,
        },
    );

    if let Ok(actual) = result {
        assert_eq!(
            actual,
            expected,
            "data/idna/test/IdnaTestV2.txt:{}",
            line_idx + 1
        );

        if status.starts_with("[C") {
            // http://unicode.org/reports/tr46/#Deviations
            // applications that perform IDNA2008 lookup are not required to check
            // for these contexts
            return;
        }

        if status == "[V2]" {
            // Everybody ignores V2
            // https://github.com/servo/rust-url/pull/240
            // https://github.com/whatwg/url/issues/53#issuecomment-181528158
            // https://www.unicode.org/review/pri317/
            return;
        }

        assert_eq!(
            status,
            "[]",
            "data/idna/test/IdnaTestV2.txt:{}",
            line_idx + 1
        );
    } else {
        assert_ne!(
            status,
            "[]",
            "data/idna/test/IdnaTestV2.txt:{}",
            line_idx + 1
        );
    }
}
