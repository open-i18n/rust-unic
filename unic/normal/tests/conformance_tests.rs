// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(test)]

use unic_normal::StrNormalForm;

type TestDatum = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);
const TEST_DATA: &[TestDatum] = include!("tables/conformance_tests_data.rsv");

#[test]
fn test_nfc_nfd_nfkc_nfkd() {
    macro_rules! norm_string {
        ($method: ident, $input: expr) => {
            $input.$method().collect::<String>()
        };
    }

    for &(s1, s2, s3, s4, s5) in TEST_DATA {
        // these invariants come from the CONFORMANCE section of
        // https://www.unicode.org/Public/UNIDATA/NormalizationTest.txt

        // NFC
        {
            let r1 = norm_string!(nfc, s1);
            let r2 = norm_string!(nfc, s2);
            let r3 = norm_string!(nfc, s3);
            let r4 = norm_string!(nfc, s4);
            let r5 = norm_string!(nfc, s5);
            assert_eq!(s2, &r1[..]);
            assert_eq!(s2, &r2[..]);
            assert_eq!(s2, &r3[..]);
            assert_eq!(s4, &r4[..]);
            assert_eq!(s4, &r5[..]);
        }

        // NFD
        {
            let r1 = norm_string!(nfd, s1);
            let r2 = norm_string!(nfd, s2);
            let r3 = norm_string!(nfd, s3);
            let r4 = norm_string!(nfd, s4);
            let r5 = norm_string!(nfd, s5);
            assert_eq!(s3, &r1[..]);
            assert_eq!(s3, &r2[..]);
            assert_eq!(s3, &r3[..]);
            assert_eq!(s5, &r4[..]);
            assert_eq!(s5, &r5[..]);
        }

        // NFKC
        {
            let r1 = norm_string!(nfkc, s1);
            let r2 = norm_string!(nfkc, s2);
            let r3 = norm_string!(nfkc, s3);
            let r4 = norm_string!(nfkc, s4);
            let r5 = norm_string!(nfkc, s5);
            assert_eq!(s4, &r1[..]);
            assert_eq!(s4, &r2[..]);
            assert_eq!(s4, &r3[..]);
            assert_eq!(s4, &r4[..]);
            assert_eq!(s4, &r5[..]);
        }

        // NFKD
        {
            let r1 = norm_string!(nfkd, s1);
            let r2 = norm_string!(nfkd, s2);
            let r3 = norm_string!(nfkd, s3);
            let r4 = norm_string!(nfkd, s4);
            let r5 = norm_string!(nfkd, s5);
            assert_eq!(s5, &r1[..]);
            assert_eq!(s5, &r2[..]);
            assert_eq!(s5, &r3[..]);
            assert_eq!(s5, &r4[..]);
            assert_eq!(s5, &r5[..]);
        }
    }
}
