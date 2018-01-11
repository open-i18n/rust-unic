// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode *Alphabetic* Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [*Alphabetic*](https://www.unicode.org/reports/tr44/#Alphabetic).
    ///
    /// The value is `true` for characters that change when lowercased, `false` otherwise.
    pub struct Alphabetic(bool) {
        abbr => "Alpha";
        long => "Alphabetic";
        human => "Alphabetic";

        data_table_path => "../tables/alphabetic.rsv";
    }

    /// Return `true` for Alphabetic characters, `false` otherwise.
    pub fn is_alphabetic(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_alphabetic;

        // ASCII
        assert_eq!(is_alphabetic('\u{0020}'), false);
        assert_eq!(is_alphabetic('\u{0021}'), false);
        assert_eq!(is_alphabetic('\u{0022}'), false);

        assert_eq!(is_alphabetic('\u{0030}'), false);
        assert_eq!(is_alphabetic('\u{0031}'), false);
        assert_eq!(is_alphabetic('\u{0032}'), false);

        assert_eq!(is_alphabetic('\u{0040}'), false);
        assert_eq!(is_alphabetic('\u{0041}'), true);
        assert_eq!(is_alphabetic('\u{0042}'), true);

        assert_eq!(is_alphabetic('\u{0060}'), false);
        assert_eq!(is_alphabetic('\u{0061}'), true);
        assert_eq!(is_alphabetic('\u{0062}'), true);

        assert_eq!(is_alphabetic('\u{007e}'), false);
        assert_eq!(is_alphabetic('\u{007f}'), false);

        // Other BMP
        assert_eq!(is_alphabetic('\u{061b}'), false);
        assert_eq!(is_alphabetic('\u{061c}'), false);
        assert_eq!(is_alphabetic('\u{061d}'), false);

        assert_eq!(is_alphabetic('\u{200d}'), false);
        assert_eq!(is_alphabetic('\u{200e}'), false);
        assert_eq!(is_alphabetic('\u{200f}'), false);
        assert_eq!(is_alphabetic('\u{2010}'), false);

        assert_eq!(is_alphabetic('\u{2029}'), false);
        assert_eq!(is_alphabetic('\u{202a}'), false);
        assert_eq!(is_alphabetic('\u{202e}'), false);
        assert_eq!(is_alphabetic('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_alphabetic('\u{10000}'), true);
        assert_eq!(is_alphabetic('\u{10001}'), true);

        assert_eq!(is_alphabetic('\u{20000}'), true);
        assert_eq!(is_alphabetic('\u{30000}'), false);
        assert_eq!(is_alphabetic('\u{40000}'), false);
        assert_eq!(is_alphabetic('\u{50000}'), false);
        assert_eq!(is_alphabetic('\u{60000}'), false);
        assert_eq!(is_alphabetic('\u{70000}'), false);
        assert_eq!(is_alphabetic('\u{80000}'), false);
        assert_eq!(is_alphabetic('\u{90000}'), false);
        assert_eq!(is_alphabetic('\u{a0000}'), false);
        assert_eq!(is_alphabetic('\u{b0000}'), false);
        assert_eq!(is_alphabetic('\u{c0000}'), false);
        assert_eq!(is_alphabetic('\u{d0000}'), false);
        assert_eq!(is_alphabetic('\u{e0000}'), false);

        assert_eq!(is_alphabetic('\u{efffe}'), false);
        assert_eq!(is_alphabetic('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_alphabetic('\u{f0000}'), false);
        assert_eq!(is_alphabetic('\u{f0001}'), false);
        assert_eq!(is_alphabetic('\u{ffffe}'), false);
        assert_eq!(is_alphabetic('\u{fffff}'), false);
        assert_eq!(is_alphabetic('\u{100000}'), false);
        assert_eq!(is_alphabetic('\u{100001}'), false);
        assert_eq!(is_alphabetic('\u{10fffe}'), false);
        assert_eq!(is_alphabetic('\u{10ffff}'), false);
    }
}
