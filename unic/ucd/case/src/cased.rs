// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Cased` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Cased`](https://www.unicode.org/reports/tr44/#Cased).
    ///
    /// The value is `true` for cased characters, `false` otherwise.
    pub struct Cased(bool) {
        abbr => "Cased";
        long => "Cased";
        human => "Cased";

        data_table_path => "../tables/cased.rsv";
    }

    /// Return `true` for cased character, `false` otherwise.
    pub fn is_cased(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_cased;

        // ASCII
        assert_eq!(is_cased('\u{0020}'), false);
        assert_eq!(is_cased('\u{0021}'), false);
        assert_eq!(is_cased('\u{0022}'), false);

        assert_eq!(is_cased('\u{0030}'), false);
        assert_eq!(is_cased('\u{0031}'), false);
        assert_eq!(is_cased('\u{0032}'), false);

        assert_eq!(is_cased('\u{0040}'), false);
        assert_eq!(is_cased('\u{0041}'), true);
        assert_eq!(is_cased('\u{0042}'), true);

        assert_eq!(is_cased('\u{0060}'), false);
        assert_eq!(is_cased('\u{0061}'), true);
        assert_eq!(is_cased('\u{0062}'), true);

        assert_eq!(is_cased('\u{007e}'), false);
        assert_eq!(is_cased('\u{007f}'), false);

        // Other BMP
        assert_eq!(is_cased('\u{061b}'), false);
        assert_eq!(is_cased('\u{061c}'), false);
        assert_eq!(is_cased('\u{061d}'), false);

        assert_eq!(is_cased('\u{200d}'), false);
        assert_eq!(is_cased('\u{200e}'), false);
        assert_eq!(is_cased('\u{200f}'), false);
        assert_eq!(is_cased('\u{2010}'), false);

        assert_eq!(is_cased('\u{2029}'), false);
        assert_eq!(is_cased('\u{202a}'), false);
        assert_eq!(is_cased('\u{202e}'), false);
        assert_eq!(is_cased('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_cased('\u{10000}'), false);
        assert_eq!(is_cased('\u{10001}'), false);

        assert_eq!(is_cased('\u{20000}'), false);
        assert_eq!(is_cased('\u{30000}'), false);
        assert_eq!(is_cased('\u{40000}'), false);
        assert_eq!(is_cased('\u{50000}'), false);
        assert_eq!(is_cased('\u{60000}'), false);
        assert_eq!(is_cased('\u{70000}'), false);
        assert_eq!(is_cased('\u{80000}'), false);
        assert_eq!(is_cased('\u{90000}'), false);
        assert_eq!(is_cased('\u{a0000}'), false);
        assert_eq!(is_cased('\u{b0000}'), false);
        assert_eq!(is_cased('\u{c0000}'), false);
        assert_eq!(is_cased('\u{d0000}'), false);
        assert_eq!(is_cased('\u{e0000}'), false);

        assert_eq!(is_cased('\u{efffe}'), false);
        assert_eq!(is_cased('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_cased('\u{f0000}'), false);
        assert_eq!(is_cased('\u{f0001}'), false);
        assert_eq!(is_cased('\u{ffffe}'), false);
        assert_eq!(is_cased('\u{fffff}'), false);
        assert_eq!(is_cased('\u{100000}'), false);
        assert_eq!(is_cased('\u{100001}'), false);
        assert_eq!(is_cased('\u{10fffe}'), false);
        assert_eq!(is_cased('\u{10ffff}'), false);
    }
}
