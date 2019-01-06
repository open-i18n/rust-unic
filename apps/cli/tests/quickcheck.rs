// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate quickcheck;



use unic_cli::parsers;
use unic_cli::writers;

// Quickcheck parsers against writers
quickcheck! {
    fn quickcheck_codepoints(input: String) -> bool {
        // 6 bytes for each BMP char, plus 1 SPACE
        let mut printed = Vec::with_capacity(input.len() * 7);
        writers::write_as_codepoints(&mut printed, input.chars()).unwrap();
        let output = parsers::codepoints(&String::from_utf8(printed).unwrap());
        input == output
    }

    fn quickcheck_utf8_hex(input: String) -> bool {
        // 4 bytes for each ASCII char, plus 1 SPACE
        let mut printed = Vec::with_capacity(input.len() * 5);
        writers::write_as_utf8_hex(&mut printed, input.chars()).unwrap();
        let output = parsers::utf8_hex(&String::from_utf8(printed).unwrap());
        input == output
    }

    fn quickcheck_utf16_hex(input: String) -> bool {
        // 6 bytes for each BMP char, plus 1 SPACE
        let mut printed = Vec::with_capacity(input.len() * 7);
        writers::write_as_utf16_hex(&mut printed, input.chars()).unwrap();
        let output = parsers::utf16_hex(&String::from_utf8(printed).unwrap());
        input == output
    }
}
