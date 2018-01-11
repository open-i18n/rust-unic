// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Unicode `Bidi_Mirrored` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Bidi_Mirrored`](https://www.unicode.org/reports/tr44/#Bidi_Mirrored).
    ///
    /// The value is `true` if the character is a "mirrored" character in bidirectional context,
    /// `false` otherwise.
    pub struct BidiMirrored(bool) {
        abbr => "Bidi_M";
        long => "Bidi_Mirrored";
        human => "Bidi Mirrored";

        data_table_path => "../tables/bidi_mirrored.rsv";
    }

    /// Return `true` if the character is a "mirrored" character in bidirectional context, `false`
    /// otherwise.
    pub fn is_bidi_mirrored(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_bidi_mirrored;

        // ASCII
        assert_eq!(is_bidi_mirrored('\u{0000}'), false);
        assert_eq!(is_bidi_mirrored('\u{0021}'), false);

        assert_eq!(is_bidi_mirrored('\u{0027}'), false);
        assert_eq!(is_bidi_mirrored('\u{0028}'), true);
        assert_eq!(is_bidi_mirrored('\u{0029}'), true);
        assert_eq!(is_bidi_mirrored('\u{002a}'), false);

        assert_eq!(is_bidi_mirrored('\u{003b}'), false);
        assert_eq!(is_bidi_mirrored('\u{003c}'), true);
        assert_eq!(is_bidi_mirrored('\u{003d}'), false);

        assert_eq!(is_bidi_mirrored('\u{007a}'), false);
        assert_eq!(is_bidi_mirrored('\u{007b}'), true);
        assert_eq!(is_bidi_mirrored('\u{007c}'), false);
        assert_eq!(is_bidi_mirrored('\u{007d}'), true);
        assert_eq!(is_bidi_mirrored('\u{007e}'), false);

        // Other BMP
        assert_eq!(is_bidi_mirrored('\u{0f39}'), false);
        assert_eq!(is_bidi_mirrored('\u{0f3a}'), true);
        assert_eq!(is_bidi_mirrored('\u{0f3b}'), true);
        assert_eq!(is_bidi_mirrored('\u{0f3c}'), true);
        assert_eq!(is_bidi_mirrored('\u{0f3d}'), true);
        assert_eq!(is_bidi_mirrored('\u{0f3e}'), false);
        assert_eq!(is_bidi_mirrored('\u{0f3f}'), false);

        assert_eq!(is_bidi_mirrored('\u{ff5c}'), false);
        assert_eq!(is_bidi_mirrored('\u{ff5d}'), true);
        assert_eq!(is_bidi_mirrored('\u{ff5e}'), false);
        assert_eq!(is_bidi_mirrored('\u{ff5f}'), true);
        assert_eq!(is_bidi_mirrored('\u{ff60}'), true);
        assert_eq!(is_bidi_mirrored('\u{ff61}'), false);

        assert_eq!(is_bidi_mirrored('\u{ff61}'), false);
        assert_eq!(is_bidi_mirrored('\u{ff62}'), true);
        assert_eq!(is_bidi_mirrored('\u{ff63}'), true);
        assert_eq!(is_bidi_mirrored('\u{ff64}'), false);

        // Other Planes
        assert_eq!(is_bidi_mirrored('\u{10000}'), false);
        assert_eq!(is_bidi_mirrored('\u{10001}'), false);

        assert_eq!(is_bidi_mirrored('\u{20000}'), false);
        assert_eq!(is_bidi_mirrored('\u{30000}'), false);
        assert_eq!(is_bidi_mirrored('\u{40000}'), false);
        assert_eq!(is_bidi_mirrored('\u{50000}'), false);
        assert_eq!(is_bidi_mirrored('\u{60000}'), false);
        assert_eq!(is_bidi_mirrored('\u{70000}'), false);
        assert_eq!(is_bidi_mirrored('\u{80000}'), false);
        assert_eq!(is_bidi_mirrored('\u{90000}'), false);
        assert_eq!(is_bidi_mirrored('\u{a0000}'), false);
        assert_eq!(is_bidi_mirrored('\u{b0000}'), false);
        assert_eq!(is_bidi_mirrored('\u{c0000}'), false);
        assert_eq!(is_bidi_mirrored('\u{d0000}'), false);
        assert_eq!(is_bidi_mirrored('\u{e0000}'), false);

        assert_eq!(is_bidi_mirrored('\u{efffe}'), false);
        assert_eq!(is_bidi_mirrored('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_bidi_mirrored('\u{f0000}'), false);
        assert_eq!(is_bidi_mirrored('\u{f0001}'), false);
        assert_eq!(is_bidi_mirrored('\u{ffffe}'), false);
        assert_eq!(is_bidi_mirrored('\u{fffff}'), false);
        assert_eq!(is_bidi_mirrored('\u{100000}'), false);
        assert_eq!(is_bidi_mirrored('\u{100001}'), false);
        assert_eq!(is_bidi_mirrored('\u{10fffe}'), false);
        assert_eq!(is_bidi_mirrored('\u{10ffff}'), false);
    }
}
