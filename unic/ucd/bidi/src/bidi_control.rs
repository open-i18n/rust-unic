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

//! Unicode `Bidi_Control` Character Property.

char_property! {
    /// Represents values of the Unicode character property
    /// [`Bidi_Control`](https://www.unicode.org/reports/tr44/#Bidi_Control).
    ///
    /// The value is `true` if the character is a Bidirectional control character, `false`
    /// otherwise.
    pub struct BidiControl(bool) {
        abbr => "Bidi_C";
        long => "Bidi_Control";
        human => "Bidi Control";

        data_table_path => "../tables/bidi_control.rsv";
    }

    /// Return `true` if the character is a Bidirectional control character, `false` otherwise.
    pub fn is_bidi_control(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_bidi_control;

        // ASCII
        assert_eq!(is_bidi_control('\u{0000}'), false);
        assert_eq!(is_bidi_control('\u{0021}'), false);

        assert_eq!(is_bidi_control('\u{0027}'), false);
        assert_eq!(is_bidi_control('\u{0028}'), false);
        assert_eq!(is_bidi_control('\u{0029}'), false);
        assert_eq!(is_bidi_control('\u{002a}'), false);

        assert_eq!(is_bidi_control('\u{003b}'), false);
        assert_eq!(is_bidi_control('\u{003c}'), false);
        assert_eq!(is_bidi_control('\u{003d}'), false);

        assert_eq!(is_bidi_control('\u{007a}'), false);
        assert_eq!(is_bidi_control('\u{007b}'), false);
        assert_eq!(is_bidi_control('\u{007c}'), false);
        assert_eq!(is_bidi_control('\u{007d}'), false);
        assert_eq!(is_bidi_control('\u{007e}'), false);

        // Other BMP
        assert_eq!(is_bidi_control('\u{061b}'), false);
        assert_eq!(is_bidi_control('\u{061c}'), true);
        assert_eq!(is_bidi_control('\u{061d}'), false);

        assert_eq!(is_bidi_control('\u{200d}'), false);
        assert_eq!(is_bidi_control('\u{200e}'), true);
        assert_eq!(is_bidi_control('\u{200f}'), true);
        assert_eq!(is_bidi_control('\u{2010}'), false);

        assert_eq!(is_bidi_control('\u{2029}'), false);
        assert_eq!(is_bidi_control('\u{202a}'), true);
        assert_eq!(is_bidi_control('\u{202e}'), true);
        assert_eq!(is_bidi_control('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_bidi_control('\u{10000}'), false);
        assert_eq!(is_bidi_control('\u{10001}'), false);

        assert_eq!(is_bidi_control('\u{20000}'), false);
        assert_eq!(is_bidi_control('\u{30000}'), false);
        assert_eq!(is_bidi_control('\u{40000}'), false);
        assert_eq!(is_bidi_control('\u{50000}'), false);
        assert_eq!(is_bidi_control('\u{60000}'), false);
        assert_eq!(is_bidi_control('\u{70000}'), false);
        assert_eq!(is_bidi_control('\u{80000}'), false);
        assert_eq!(is_bidi_control('\u{90000}'), false);
        assert_eq!(is_bidi_control('\u{a0000}'), false);
        assert_eq!(is_bidi_control('\u{b0000}'), false);
        assert_eq!(is_bidi_control('\u{c0000}'), false);
        assert_eq!(is_bidi_control('\u{d0000}'), false);
        assert_eq!(is_bidi_control('\u{e0000}'), false);

        assert_eq!(is_bidi_control('\u{efffe}'), false);
        assert_eq!(is_bidi_control('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_bidi_control('\u{f0000}'), false);
        assert_eq!(is_bidi_control('\u{f0001}'), false);
        assert_eq!(is_bidi_control('\u{ffffe}'), false);
        assert_eq!(is_bidi_control('\u{fffff}'), false);
        assert_eq!(is_bidi_control('\u{100000}'), false);
        assert_eq!(is_bidi_control('\u{100001}'), false);
        assert_eq!(is_bidi_control('\u{10fffe}'), false);
        assert_eq!(is_bidi_control('\u{10ffff}'), false);
    }
}
