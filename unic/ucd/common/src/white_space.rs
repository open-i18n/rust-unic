// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `White_Space` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [*White_Space*](https://www.unicode.org/reports/tr44/#White_Space).
    ///
    /// The value is `true` for characters that change when lowercased, `false` otherwise.
    pub struct WhiteSpace(bool) {
        abbr => "WSpace";
        long => "White_Space";
        human => "White Space";

        data_table_path => "../tables/white_space.rsv";
    }

    /// Return `true` for white-space characters, `false` otherwise.
    pub fn is_white_space(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_white_space;

        // ASCII
        assert_eq!(is_white_space('\u{0020}'), true);
        assert_eq!(is_white_space('\u{0021}'), false);
        assert_eq!(is_white_space('\u{0022}'), false);

        assert_eq!(is_white_space('\u{0030}'), false);
        assert_eq!(is_white_space('\u{0031}'), false);
        assert_eq!(is_white_space('\u{0032}'), false);

        assert_eq!(is_white_space('\u{0040}'), false);
        assert_eq!(is_white_space('\u{0041}'), false);
        assert_eq!(is_white_space('\u{0042}'), false);

        assert_eq!(is_white_space('\u{0060}'), false);
        assert_eq!(is_white_space('\u{0061}'), false);
        assert_eq!(is_white_space('\u{0062}'), false);

        assert_eq!(is_white_space('\u{007e}'), false);
        assert_eq!(is_white_space('\u{007f}'), false);

        // Other BMP
        assert_eq!(is_white_space('\u{061b}'), false);
        assert_eq!(is_white_space('\u{061c}'), false);
        assert_eq!(is_white_space('\u{061d}'), false);

        assert_eq!(is_white_space('\u{200d}'), false);
        assert_eq!(is_white_space('\u{200e}'), false);
        assert_eq!(is_white_space('\u{200f}'), false);
        assert_eq!(is_white_space('\u{2010}'), false);

        assert_eq!(is_white_space('\u{2029}'), true);
        assert_eq!(is_white_space('\u{202a}'), false);
        assert_eq!(is_white_space('\u{202e}'), false);
        assert_eq!(is_white_space('\u{202f}'), true);

        // Other Planes
        assert_eq!(is_white_space('\u{10000}'), false);
        assert_eq!(is_white_space('\u{10001}'), false);

        assert_eq!(is_white_space('\u{20000}'), false);
        assert_eq!(is_white_space('\u{30000}'), false);
        assert_eq!(is_white_space('\u{40000}'), false);
        assert_eq!(is_white_space('\u{50000}'), false);
        assert_eq!(is_white_space('\u{60000}'), false);
        assert_eq!(is_white_space('\u{70000}'), false);
        assert_eq!(is_white_space('\u{80000}'), false);
        assert_eq!(is_white_space('\u{90000}'), false);
        assert_eq!(is_white_space('\u{a0000}'), false);
        assert_eq!(is_white_space('\u{b0000}'), false);
        assert_eq!(is_white_space('\u{c0000}'), false);
        assert_eq!(is_white_space('\u{d0000}'), false);
        assert_eq!(is_white_space('\u{e0000}'), false);

        assert_eq!(is_white_space('\u{efffe}'), false);
        assert_eq!(is_white_space('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_white_space('\u{f0000}'), false);
        assert_eq!(is_white_space('\u{f0001}'), false);
        assert_eq!(is_white_space('\u{ffffe}'), false);
        assert_eq!(is_white_space('\u{fffff}'), false);
        assert_eq!(is_white_space('\u{100000}'), false);
        assert_eq!(is_white_space('\u{100001}'), false);
        assert_eq!(is_white_space('\u{10fffe}'), false);
        assert_eq!(is_white_space('\u{10ffff}'), false);
    }
}
