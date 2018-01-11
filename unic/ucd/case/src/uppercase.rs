// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Uppercase` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Uppercase`](https://www.unicode.org/reports/tr44/#Uppercase).
    ///
    /// The value is `true` for uppercase characters, `false` otherwise.
    pub struct Uppercase(bool) {
        abbr => "Upper";
        long => "Uppercase";
        human => "Uppercase";

        data_table_path => "../tables/uppercase.rsv";
    }

    /// Return `true` for uppercase character, `false` otherwise.
    pub fn is_uppercase(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_uppercase;

        // ASCII
        assert_eq!(is_uppercase('\u{0000}'), false);

        assert_eq!(is_uppercase('\u{0020}'), false);
        assert_eq!(is_uppercase('\u{0021}'), false);
        assert_eq!(is_uppercase('\u{0022}'), false);

        assert_eq!(is_uppercase('\u{0030}'), false);
        assert_eq!(is_uppercase('\u{0031}'), false);
        assert_eq!(is_uppercase('\u{0032}'), false);

        assert_eq!(is_uppercase('\u{0040}'), false);
        assert_eq!(is_uppercase('\u{0041}'), true);
        assert_eq!(is_uppercase('\u{0042}'), true);

        assert_eq!(is_uppercase('\u{0060}'), false);
        assert_eq!(is_uppercase('\u{0061}'), false);
        assert_eq!(is_uppercase('\u{0062}'), false);

        assert_eq!(is_uppercase('\u{007e}'), false);
        assert_eq!(is_uppercase('\u{007f}'), false);

        // Other Planes
        assert_eq!(is_uppercase('\u{10000}'), false);
        assert_eq!(is_uppercase('\u{10001}'), false);

        assert_eq!(is_uppercase('\u{20000}'), false);
        assert_eq!(is_uppercase('\u{30000}'), false);
        assert_eq!(is_uppercase('\u{40000}'), false);
        assert_eq!(is_uppercase('\u{50000}'), false);
        assert_eq!(is_uppercase('\u{60000}'), false);
        assert_eq!(is_uppercase('\u{70000}'), false);
        assert_eq!(is_uppercase('\u{80000}'), false);
        assert_eq!(is_uppercase('\u{90000}'), false);
        assert_eq!(is_uppercase('\u{a0000}'), false);
        assert_eq!(is_uppercase('\u{b0000}'), false);
        assert_eq!(is_uppercase('\u{c0000}'), false);
        assert_eq!(is_uppercase('\u{d0000}'), false);
        assert_eq!(is_uppercase('\u{e0000}'), false);

        assert_eq!(is_uppercase('\u{efffe}'), false);
        assert_eq!(is_uppercase('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_uppercase('\u{f0000}'), false);
        assert_eq!(is_uppercase('\u{f0001}'), false);
        assert_eq!(is_uppercase('\u{ffffe}'), false);
        assert_eq!(is_uppercase('\u{fffff}'), false);
        assert_eq!(is_uppercase('\u{100000}'), false);
        assert_eq!(is_uppercase('\u{100001}'), false);
        assert_eq!(is_uppercase('\u{10fffe}'), false);
        assert_eq!(is_uppercase('\u{10ffff}'), false);
    }
}
