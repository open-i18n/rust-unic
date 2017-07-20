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

use unic_idna_punycode::{decode, encode_str};
use rustc_serialize::json::{Json, Object};
use test::TestFn;

fn one_test(decoded: &str, encoded: &str) {
    match decode(encoded) {
        None => panic!("Decoding {} failed.", encoded),
        Some(result) => {
            let result = result.into_iter().collect::<String>();
            assert_eq!(
                result,
                decoded,
                "Incorrect decoding of \"{}\":\n   \"{}\"\n!= \"{}\"\n",
                encoded,
                result,
                decoded
            )
        }
    }

    match encode_str(decoded) {
        None => panic!("Encoding {} failed.", decoded),
        Some(result) => {
            assert_eq!(
                result,
                encoded,
                "Incorrect encoding of \"{}\":\n   \"{}\"\n!= \"{}\"\n",
                decoded,
                result,
                encoded
            )
        }
    }
}

fn get_string<'a>(map: &'a Object, key: &str) -> &'a str {
    match map.get(&key.to_string()) {
        Some(&Json::String(ref s)) => s,
        None => "",
        _ => panic!(),
    }
}

pub fn collect_tests<F: FnMut(String, TestFn)>(add_test: &mut F) {
    match Json::from_str(include_str!(
        "../../../../data/punycode/test/punycode.js_data.json"
    )) {
        Ok(Json::Array(tests)) => {
            for (i, test) in tests.into_iter().enumerate() {
                match test {
                    Json::Object(o) => {
                        let test_name = {
                            let desc = get_string(&o, "description");
                            if desc.is_empty() {
                                format!("Punycode {}", i + 1)
                            } else {
                                format!("Punycode {}: {}", i + 1, desc)
                            }
                        };
                        add_test(
                            test_name,
                            TestFn::dyn_test_fn(move || {
                                one_test(get_string(&o, "decoded"), get_string(&o, "encoded"))
                            }),
                        )
                    }
                    _ => panic!(),
                }
            }
        }
        other => panic!("{:?}", other),
    }
}
