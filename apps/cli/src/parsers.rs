// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::char;
use std::str;

use regex::Regex;

lazy_static! {
    // Anything not alphanumeric or `+`
    static ref CODEPOINT_SEPARATORS: Regex = Regex::new(r#"[^\w+]"#).unwrap();

    // Unicode codepoint prefix: `U+`
    static ref CODEPOINT_PREFIX: Regex = Regex::new(r#"^[Uu][+]"#).unwrap();

    // Anything not alphanumeric
    static ref HEX_SEPARATORS: Regex = Regex::new(r#"[^\w]"#).unwrap();

    // Hex prefix: `0x`
    static ref HEX_PREFIX: Regex = Regex::new(r#"^0[xX]"#).unwrap();
}

pub fn codepoints(string: &str) -> String {
    CODEPOINT_SEPARATORS
        .split(&string)
        .map(|token| {
            let mut token = token;
            if CODEPOINT_PREFIX.is_match(token) {
                token = &token[2..];
            }
            let codepoint = u32::from_str_radix(token, 16)
                .unwrap_or_else(|_| panic!("Cannot parse token as hex number: {}", token));
            char::from_u32(codepoint)
                .unwrap_or_else(|| panic!("Invalid Unicode Scalar Value code-point: {}", codepoint))
        })
        .collect::<String>()
}

pub fn utf8_hex(string: &str) -> String {
    let utf8 = HEX_SEPARATORS.split(&string).map(|token| {
        let mut token = token;
        if HEX_PREFIX.is_match(token) {
            token = &token[2..];
        }
        u8::from_str_radix(token, 16)
            .unwrap_or_else(|_| panic!("Cannot parse token as hex byte value: {}", token))
    });

    String::from_utf8(utf8.collect()).expect("Invalid UTF-8 sequence")
}

pub fn utf16_hex(string: &str) -> String {
    let utf16 = HEX_SEPARATORS.split(&string).map(|token| {
        let mut token = token;
        if HEX_PREFIX.is_match(token) {
            token = &token[2..];
        }
        u16::from_str_radix(token, 16)
            .unwrap_or_else(|_| panic!("Cannot parse token as hex byte value: {}", token))
    });

    char::decode_utf16(utf16)
        .map(|r| r.expect("Invalid UTF-16 sequence"))
        .collect()
}
