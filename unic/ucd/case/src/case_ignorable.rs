// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Case_Ignorable` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Case_Ignorable`](https://www.unicode.org/reports/tr44/#Case_Ignorable).
    ///
    /// The value is `true` for case-ignorable characters, `false` otherwise.
    pub struct CaseIgnorable(bool) {
        abbr => "CI";
        long => "Case_Ignorable";
        human => "Case-Ignorable";

        data_table_path => "../tables/case_ignorable.rsv";
    }

    /// Return `true` for case-ignorable character, `false` otherwise.
    pub fn is_case_ignorable(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_case_ignorable;

        // ASCII
        assert_eq!(is_case_ignorable('\u{0020}'), false);
        assert_eq!(is_case_ignorable('\u{0021}'), false);
        assert_eq!(is_case_ignorable('\u{0022}'), false);

        assert_eq!(is_case_ignorable('\u{0030}'), false);
        assert_eq!(is_case_ignorable('\u{0031}'), false);
        assert_eq!(is_case_ignorable('\u{0032}'), false);

        assert_eq!(is_case_ignorable('\u{0040}'), false);
        assert_eq!(is_case_ignorable('\u{0041}'), false);
        assert_eq!(is_case_ignorable('\u{0042}'), false);

        assert_eq!(is_case_ignorable('\u{0060}'), true);
        assert_eq!(is_case_ignorable('\u{0061}'), false);
        assert_eq!(is_case_ignorable('\u{0062}'), false);

        assert_eq!(is_case_ignorable('\u{007e}'), false);
        assert_eq!(is_case_ignorable('\u{007f}'), false);

        // Other BMP
        assert_eq!(is_case_ignorable('\u{061b}'), false);
        assert_eq!(is_case_ignorable('\u{061c}'), true);
        assert_eq!(is_case_ignorable('\u{061d}'), false);

        assert_eq!(is_case_ignorable('\u{200d}'), true);
        assert_eq!(is_case_ignorable('\u{200e}'), true);
        assert_eq!(is_case_ignorable('\u{200f}'), true);
        assert_eq!(is_case_ignorable('\u{2010}'), false);

        assert_eq!(is_case_ignorable('\u{2029}'), false);
        assert_eq!(is_case_ignorable('\u{202a}'), true);
        assert_eq!(is_case_ignorable('\u{202e}'), true);
        assert_eq!(is_case_ignorable('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_case_ignorable('\u{10000}'), false);
        assert_eq!(is_case_ignorable('\u{10001}'), false);

        assert_eq!(is_case_ignorable('\u{20000}'), false);
        assert_eq!(is_case_ignorable('\u{30000}'), false);
        assert_eq!(is_case_ignorable('\u{40000}'), false);
        assert_eq!(is_case_ignorable('\u{50000}'), false);
        assert_eq!(is_case_ignorable('\u{60000}'), false);
        assert_eq!(is_case_ignorable('\u{70000}'), false);
        assert_eq!(is_case_ignorable('\u{80000}'), false);
        assert_eq!(is_case_ignorable('\u{90000}'), false);
        assert_eq!(is_case_ignorable('\u{a0000}'), false);
        assert_eq!(is_case_ignorable('\u{b0000}'), false);
        assert_eq!(is_case_ignorable('\u{c0000}'), false);
        assert_eq!(is_case_ignorable('\u{d0000}'), false);
        assert_eq!(is_case_ignorable('\u{e0000}'), false);

        assert_eq!(is_case_ignorable('\u{efffe}'), false);
        assert_eq!(is_case_ignorable('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_case_ignorable('\u{f0000}'), false);
        assert_eq!(is_case_ignorable('\u{f0001}'), false);
        assert_eq!(is_case_ignorable('\u{ffffe}'), false);
        assert_eq!(is_case_ignorable('\u{fffff}'), false);
        assert_eq!(is_case_ignorable('\u{100000}'), false);
        assert_eq!(is_case_ignorable('\u{100001}'), false);
        assert_eq!(is_case_ignorable('\u{10fffe}'), false);
        assert_eq!(is_case_ignorable('\u{10ffff}'), false);
    }
}
