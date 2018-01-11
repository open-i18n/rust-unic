// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! *Control* Character Property, equal to `General_Category = Cc`.
//!
//! NOTE: This property is not defined by UCD, but is used commonly enough in Unicode algorithms and
//! applications to provide an optimized implementation.

char_property! {
    /// Represents Unicode characters with `General_Category = Cc`.
    ///
    /// The value is `true` for characters that have a control (`Cc`) *General_Category*, `false`
    /// otherwise.
    pub struct Control(bool) {
        abbr => "Control";
        long => "Control";
        human => "Control";

        data_table_path => "../tables/control.rsv";
    }

    /// Return `true` for control characters, `false` otherwise.
    pub fn is_control(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_control;

        // ASCII
        assert_eq!(is_control('\u{0000}'), true);
        assert_eq!(is_control('\u{0001}'), true);
        assert_eq!(is_control('\u{0002}'), true);

        assert_eq!(is_control('\u{0010}'), true);
        assert_eq!(is_control('\u{0011}'), true);
        assert_eq!(is_control('\u{0012}'), true);

        assert_eq!(is_control('\u{0020}'), false);
        assert_eq!(is_control('\u{0021}'), false);
        assert_eq!(is_control('\u{0022}'), false);

        assert_eq!(is_control('\u{0030}'), false);
        assert_eq!(is_control('\u{0031}'), false);
        assert_eq!(is_control('\u{0032}'), false);

        assert_eq!(is_control('\u{0040}'), false);
        assert_eq!(is_control('\u{0041}'), false);
        assert_eq!(is_control('\u{0042}'), false);

        assert_eq!(is_control('\u{0060}'), false);
        assert_eq!(is_control('\u{0061}'), false);
        assert_eq!(is_control('\u{0062}'), false);

        assert_eq!(is_control('\u{007e}'), false);
        assert_eq!(is_control('\u{007f}'), true);

        // Other BMP
        assert_eq!(is_control('\u{061b}'), false);
        assert_eq!(is_control('\u{061c}'), false);
        assert_eq!(is_control('\u{061d}'), false);

        assert_eq!(is_control('\u{200d}'), false);
        assert_eq!(is_control('\u{200e}'), false);
        assert_eq!(is_control('\u{200f}'), false);
        assert_eq!(is_control('\u{2010}'), false);

        assert_eq!(is_control('\u{2029}'), false);
        assert_eq!(is_control('\u{202a}'), false);
        assert_eq!(is_control('\u{202e}'), false);
        assert_eq!(is_control('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_control('\u{10000}'), false);
        assert_eq!(is_control('\u{10001}'), false);

        assert_eq!(is_control('\u{20000}'), false);
        assert_eq!(is_control('\u{30000}'), false);
        assert_eq!(is_control('\u{40000}'), false);
        assert_eq!(is_control('\u{50000}'), false);
        assert_eq!(is_control('\u{60000}'), false);
        assert_eq!(is_control('\u{70000}'), false);
        assert_eq!(is_control('\u{80000}'), false);
        assert_eq!(is_control('\u{90000}'), false);
        assert_eq!(is_control('\u{a0000}'), false);
        assert_eq!(is_control('\u{b0000}'), false);
        assert_eq!(is_control('\u{c0000}'), false);
        assert_eq!(is_control('\u{d0000}'), false);
        assert_eq!(is_control('\u{e0000}'), false);

        assert_eq!(is_control('\u{efffe}'), false);
        assert_eq!(is_control('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_control('\u{f0000}'), false);
        assert_eq!(is_control('\u{f0001}'), false);
        assert_eq!(is_control('\u{ffffe}'), false);
        assert_eq!(is_control('\u{fffff}'), false);
        assert_eq!(is_control('\u{100000}'), false);
        assert_eq!(is_control('\u{100001}'), false);
        assert_eq!(is_control('\u{10fffe}'), false);
        assert_eq!(is_control('\u{10ffff}'), false);
    }
}
