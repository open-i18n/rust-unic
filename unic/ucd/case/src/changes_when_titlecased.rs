// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Changes_When_Titlecased` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Changes_When_Titlecased`](https://www.unicode.org/reports/tr44/#CWT).
    ///
    /// The value is `true` for characters that change when titlecased, `false` otherwise.
    pub struct ChangesWhenTitlecased(bool) {
        abbr => "CWT";
        long => "Changes_When_Titlecased";
        human => "Changes When Titlecased";

        data_table_path => "../tables/changes_when_titlecased.rsv";
    }

    /// Return `true` for characters that change when titlecased, `false` otherwise.
    pub fn changes_when_titlecased(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::changes_when_titlecased;

        // ASCII
        assert_eq!(changes_when_titlecased('\u{0020}'), false);
        assert_eq!(changes_when_titlecased('\u{0021}'), false);
        assert_eq!(changes_when_titlecased('\u{0022}'), false);

        assert_eq!(changes_when_titlecased('\u{0030}'), false);
        assert_eq!(changes_when_titlecased('\u{0031}'), false);
        assert_eq!(changes_when_titlecased('\u{0032}'), false);

        assert_eq!(changes_when_titlecased('\u{0040}'), false);
        assert_eq!(changes_when_titlecased('\u{0041}'), false);
        assert_eq!(changes_when_titlecased('\u{0042}'), false);

        assert_eq!(changes_when_titlecased('\u{0060}'), false);
        assert_eq!(changes_when_titlecased('\u{0061}'), true);
        assert_eq!(changes_when_titlecased('\u{0062}'), true);

        assert_eq!(changes_when_titlecased('\u{007e}'), false);
        assert_eq!(changes_when_titlecased('\u{007f}'), false);

        // Other BMP
        assert_eq!(changes_when_titlecased('\u{061b}'), false);
        assert_eq!(changes_when_titlecased('\u{061c}'), false);
        assert_eq!(changes_when_titlecased('\u{061d}'), false);

        assert_eq!(changes_when_titlecased('\u{200d}'), false);
        assert_eq!(changes_when_titlecased('\u{200e}'), false);
        assert_eq!(changes_when_titlecased('\u{200f}'), false);
        assert_eq!(changes_when_titlecased('\u{2010}'), false);

        assert_eq!(changes_when_titlecased('\u{2029}'), false);
        assert_eq!(changes_when_titlecased('\u{202a}'), false);
        assert_eq!(changes_when_titlecased('\u{202e}'), false);
        assert_eq!(changes_when_titlecased('\u{202f}'), false);

        // Other Planes
        assert_eq!(changes_when_titlecased('\u{10000}'), false);
        assert_eq!(changes_when_titlecased('\u{10001}'), false);

        assert_eq!(changes_when_titlecased('\u{20000}'), false);
        assert_eq!(changes_when_titlecased('\u{30000}'), false);
        assert_eq!(changes_when_titlecased('\u{40000}'), false);
        assert_eq!(changes_when_titlecased('\u{50000}'), false);
        assert_eq!(changes_when_titlecased('\u{60000}'), false);
        assert_eq!(changes_when_titlecased('\u{70000}'), false);
        assert_eq!(changes_when_titlecased('\u{80000}'), false);
        assert_eq!(changes_when_titlecased('\u{90000}'), false);
        assert_eq!(changes_when_titlecased('\u{a0000}'), false);
        assert_eq!(changes_when_titlecased('\u{b0000}'), false);
        assert_eq!(changes_when_titlecased('\u{c0000}'), false);
        assert_eq!(changes_when_titlecased('\u{d0000}'), false);
        assert_eq!(changes_when_titlecased('\u{e0000}'), false);

        assert_eq!(changes_when_titlecased('\u{efffe}'), false);
        assert_eq!(changes_when_titlecased('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(changes_when_titlecased('\u{f0000}'), false);
        assert_eq!(changes_when_titlecased('\u{f0001}'), false);
        assert_eq!(changes_when_titlecased('\u{ffffe}'), false);
        assert_eq!(changes_when_titlecased('\u{fffff}'), false);
        assert_eq!(changes_when_titlecased('\u{100000}'), false);
        assert_eq!(changes_when_titlecased('\u{100001}'), false);
        assert_eq!(changes_when_titlecased('\u{10fffe}'), false);
        assert_eq!(changes_when_titlecased('\u{10ffff}'), false);
    }
}
