// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Extended_Pictographic` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Extended_Pictograpic`](https://www.unicode.org/reports/tr51/#Emoji_Properties).
    ///
    /// The value is `true` for characters that are used to future-proof segmentation, `false`
    /// otherweise. The Extended_Pictographic characters contain all the Emoji characters except for
    /// some Emoji_Component characters.
    pub struct ExtendedPictographic(bool) {
        abbr => "Extended_Pictographic";
        long => "Extended_Pictographic";
        human => "Extended Pictographic";

        data_table_path => "../tables/extended_pictographic.rsv";
    }

    /// The value is `true` for characters that are used to future-proof segmentation, `false`
    /// otherweise. The Extended_Pictographic characters contain all the Emoji characters except for
    /// some Emoji_Component characters.
    pub fn is_extended_pictographic(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_extended_pictographic;

        // ASCII
        assert_eq!(is_extended_pictographic('\u{0000}'), false);
        assert_eq!(is_extended_pictographic('\u{0021}'), false);

        assert_eq!(is_extended_pictographic('\u{0027}'), false);
        assert_eq!(is_extended_pictographic('\u{0028}'), false);
        assert_eq!(is_extended_pictographic('\u{0029}'), false);
        assert_eq!(is_extended_pictographic('\u{002a}'), false);

        assert_eq!(is_extended_pictographic('\u{003b}'), false);
        assert_eq!(is_extended_pictographic('\u{003c}'), false);
        assert_eq!(is_extended_pictographic('\u{003d}'), false);

        assert_eq!(is_extended_pictographic('\u{007a}'), false);
        assert_eq!(is_extended_pictographic('\u{007b}'), false);
        assert_eq!(is_extended_pictographic('\u{007c}'), false);
        assert_eq!(is_extended_pictographic('\u{007d}'), false);
        assert_eq!(is_extended_pictographic('\u{007e}'), false);

        assert_eq!(is_extended_pictographic('\u{00a9}'), true);
        assert_eq!(is_extended_pictographic('\u{00ae}'), true);

        // Other BMP
        assert_eq!(is_extended_pictographic('\u{061b}'), false);
        assert_eq!(is_extended_pictographic('\u{061c}'), false);
        assert_eq!(is_extended_pictographic('\u{061d}'), false);

        assert_eq!(is_extended_pictographic('\u{200d}'), false);
        assert_eq!(is_extended_pictographic('\u{200e}'), false);
        assert_eq!(is_extended_pictographic('\u{200f}'), false);
        assert_eq!(is_extended_pictographic('\u{2010}'), false);

        assert_eq!(is_extended_pictographic('\u{2029}'), false);
        assert_eq!(is_extended_pictographic('\u{202a}'), false);
        assert_eq!(is_extended_pictographic('\u{202e}'), false);
        assert_eq!(is_extended_pictographic('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_extended_pictographic('\u{10000}'), false);
        assert_eq!(is_extended_pictographic('\u{10001}'), false);

        assert_eq!(is_extended_pictographic('\u{1f1e5}'), true);
        assert_eq!(is_extended_pictographic('\u{1f1e6}'), false);
        assert_eq!(is_extended_pictographic('\u{1f1ff}'), false);
        assert_eq!(is_extended_pictographic('\u{1f200}'), false);

        assert_eq!(is_extended_pictographic('\u{20000}'), false);
        assert_eq!(is_extended_pictographic('\u{30000}'), false);
        assert_eq!(is_extended_pictographic('\u{40000}'), false);
        assert_eq!(is_extended_pictographic('\u{50000}'), false);
        assert_eq!(is_extended_pictographic('\u{60000}'), false);
        assert_eq!(is_extended_pictographic('\u{70000}'), false);
        assert_eq!(is_extended_pictographic('\u{80000}'), false);
        assert_eq!(is_extended_pictographic('\u{90000}'), false);
        assert_eq!(is_extended_pictographic('\u{a0000}'), false);
        assert_eq!(is_extended_pictographic('\u{b0000}'), false);
        assert_eq!(is_extended_pictographic('\u{c0000}'), false);
        assert_eq!(is_extended_pictographic('\u{d0000}'), false);
        assert_eq!(is_extended_pictographic('\u{e0000}'), false);

        assert_eq!(is_extended_pictographic('\u{efffe}'), false);
        assert_eq!(is_extended_pictographic('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_extended_pictographic('\u{f0000}'), false);
        assert_eq!(is_extended_pictographic('\u{f0001}'), false);
        assert_eq!(is_extended_pictographic('\u{ffffe}'), false);
        assert_eq!(is_extended_pictographic('\u{fffff}'), false);
        assert_eq!(is_extended_pictographic('\u{100000}'), false);
        assert_eq!(is_extended_pictographic('\u{100001}'), false);
        assert_eq!(is_extended_pictographic('\u{10fffe}'), false);
        assert_eq!(is_extended_pictographic('\u{10ffff}'), false);
    }
}
