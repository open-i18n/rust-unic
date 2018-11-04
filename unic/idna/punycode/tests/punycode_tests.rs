// Copyright 2013 The rust-url developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_serialize;
extern crate unic_idna_punycode;

use rustc_serialize::json::{self, Json};
use unic_idna_punycode::{decode, encode_str};

#[test]
fn test_punycode_js_data() {
    let tests = match Json::from_str(include_str!(
        "../../../../data/punycode/test/punycode.js_data.json"
    ))
    .unwrap()
    {
        Json::Array(tests) => tests,
        _ => panic!("Bad JSON tests data"),
    };

    for (test_idx, test) in tests.into_iter().enumerate() {
        match test {
            Json::Object(obj) => {
                let test_name = {
                    let desc = get_string(&obj, "description");
                    if desc.is_empty() {
                        format!("Punycode {}", test_idx + 1)
                    } else {
                        format!("Punycode {}: {}", test_idx + 1, desc)
                    }
                };
                one_test(
                    &test_name,
                    get_string(&obj, "decoded"),
                    get_string(&obj, "encoded"),
                )
            }
            _ => panic!("Bad JSON test data"),
        }
    }
}

fn one_test(test_name: &str, decoded: &str, encoded: &str) {
    match decode(encoded) {
        None => panic!("Decoding {} failed.", encoded),
        Some(result) => {
            let result = result.into_iter().collect::<String>();
            assert_eq!(
                result, decoded,
                "Test `{}`: Incorrect decoding of \"{}\":\n   \"{}\"\n!= \"{}\"\n",
                test_name, encoded, result, decoded
            )
        }
    }

    match encode_str(decoded) {
        None => panic!("Encoding {} failed.", decoded),
        Some(result) => assert_eq!(
            result, encoded,
            "Test `{}`: Incorrect encoding of \"{}\":\n   \"{}\"\n!= \"{}\"\n",
            test_name, decoded, result, encoded
        ),
    }
}

fn get_string<'a>(obj: &'a json::Object, key: &str) -> &'a str {
    match obj.get(key) {
        Some(&Json::String(ref s)) => s,
        None => "",
        _ => panic!(),
    }
}
