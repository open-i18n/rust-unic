// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Changes_When_Casefolded` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Changes_When_Casefolded`](https://www.unicode.org/reports/tr44/#CWCF).
    ///
    /// The value is `true` for characters that change when casefolded, `false` otherwise.
    pub struct ChangesWhenCasefolded(bool) {
        abbr => "CWCF";
        long => "Changes_When_Casefolded";
        human => "Changes When Casefolded";

        data_table_path => "../tables/changes_when_casefolded.rsv";
    }

    /// Return `true` for characters that change when casefolded, `false` otherwise.
    pub fn changes_when_casefolded(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::changes_when_casefolded;

        // ASCII
        assert_eq!(changes_when_casefolded('\u{0020}'), false);
        assert_eq!(changes_when_casefolded('\u{0021}'), false);
        assert_eq!(changes_when_casefolded('\u{0022}'), false);

        assert_eq!(changes_when_casefolded('\u{0030}'), false);
        assert_eq!(changes_when_casefolded('\u{0031}'), false);
        assert_eq!(changes_when_casefolded('\u{0032}'), false);

        assert_eq!(changes_when_casefolded('\u{0040}'), false);
        assert_eq!(changes_when_casefolded('\u{0041}'), true);
        assert_eq!(changes_when_casefolded('\u{0042}'), true);

        assert_eq!(changes_when_casefolded('\u{0060}'), false);
        assert_eq!(changes_when_casefolded('\u{0061}'), false);
        assert_eq!(changes_when_casefolded('\u{0062}'), false);

        assert_eq!(changes_when_casefolded('\u{007e}'), false);
        assert_eq!(changes_when_casefolded('\u{007f}'), false);

        // Other BMP
        assert_eq!(changes_when_casefolded('\u{061b}'), false);
        assert_eq!(changes_when_casefolded('\u{061c}'), false);
        assert_eq!(changes_when_casefolded('\u{061d}'), false);

        assert_eq!(changes_when_casefolded('\u{200d}'), false);
        assert_eq!(changes_when_casefolded('\u{200e}'), false);
        assert_eq!(changes_when_casefolded('\u{200f}'), false);
        assert_eq!(changes_when_casefolded('\u{2010}'), false);

        assert_eq!(changes_when_casefolded('\u{2029}'), false);
        assert_eq!(changes_when_casefolded('\u{202a}'), false);
        assert_eq!(changes_when_casefolded('\u{202e}'), false);
        assert_eq!(changes_when_casefolded('\u{202f}'), false);

        // Other Planes
        assert_eq!(changes_when_casefolded('\u{10000}'), false);
        assert_eq!(changes_when_casefolded('\u{10001}'), false);

        assert_eq!(changes_when_casefolded('\u{20000}'), false);
        assert_eq!(changes_when_casefolded('\u{30000}'), false);
        assert_eq!(changes_when_casefolded('\u{40000}'), false);
        assert_eq!(changes_when_casefolded('\u{50000}'), false);
        assert_eq!(changes_when_casefolded('\u{60000}'), false);
        assert_eq!(changes_when_casefolded('\u{70000}'), false);
        assert_eq!(changes_when_casefolded('\u{80000}'), false);
        assert_eq!(changes_when_casefolded('\u{90000}'), false);
        assert_eq!(changes_when_casefolded('\u{a0000}'), false);
        assert_eq!(changes_when_casefolded('\u{b0000}'), false);
        assert_eq!(changes_when_casefolded('\u{c0000}'), false);
        assert_eq!(changes_when_casefolded('\u{d0000}'), false);
        assert_eq!(changes_when_casefolded('\u{e0000}'), false);

        assert_eq!(changes_when_casefolded('\u{efffe}'), false);
        assert_eq!(changes_when_casefolded('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(changes_when_casefolded('\u{f0000}'), false);
        assert_eq!(changes_when_casefolded('\u{f0001}'), false);
        assert_eq!(changes_when_casefolded('\u{ffffe}'), false);
        assert_eq!(changes_when_casefolded('\u{fffff}'), false);
        assert_eq!(changes_when_casefolded('\u{100000}'), false);
        assert_eq!(changes_when_casefolded('\u{100001}'), false);
        assert_eq!(changes_when_casefolded('\u{10fffe}'), false);
        assert_eq!(changes_when_casefolded('\u{10ffff}'), false);
    }
}
