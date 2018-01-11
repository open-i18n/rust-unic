// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! *Numeric* Character Property, equal to `General_Category = Nd | Nl | No`.
//!
//! NOTE: This property is not defined by UCD, but is used commonly enough in Unicode algorithms and
//! applications to provide an optimized implementation.

char_property! {
    /// Represents Unicode characters with `General_Category = Nd | Nl | No`.
    ///
    /// The value is `true` for characters that have a numeric *General_Category*, `false`
    /// otherwise.
    pub struct Numeric(bool) {
        abbr => "Numeric";
        long => "Numeric";
        human => "Numeric";

        data_table_path => "../tables/numeric.rsv";
    }

    /// Return `true` for numeric characters, `false` otherwise.
    pub fn is_numeric(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_numeric;

        // ASCII
        assert_eq!(is_numeric('\u{0020}'), false);
        assert_eq!(is_numeric('\u{0021}'), false);
        assert_eq!(is_numeric('\u{0022}'), false);

        assert_eq!(is_numeric('\u{0030}'), true);
        assert_eq!(is_numeric('\u{0031}'), true);
        assert_eq!(is_numeric('\u{0032}'), true);

        assert_eq!(is_numeric('\u{0040}'), false);
        assert_eq!(is_numeric('\u{0041}'), false);
        assert_eq!(is_numeric('\u{0042}'), false);

        assert_eq!(is_numeric('\u{0060}'), false);
        assert_eq!(is_numeric('\u{0061}'), false);
        assert_eq!(is_numeric('\u{0062}'), false);

        assert_eq!(is_numeric('\u{007e}'), false);
        assert_eq!(is_numeric('\u{007f}'), false);

        // Other BMP
        assert_eq!(is_numeric('\u{061b}'), false);
        assert_eq!(is_numeric('\u{061c}'), false);
        assert_eq!(is_numeric('\u{061d}'), false);

        assert_eq!(is_numeric('\u{200d}'), false);
        assert_eq!(is_numeric('\u{200e}'), false);
        assert_eq!(is_numeric('\u{200f}'), false);
        assert_eq!(is_numeric('\u{2010}'), false);

        assert_eq!(is_numeric('\u{2029}'), false);
        assert_eq!(is_numeric('\u{202a}'), false);
        assert_eq!(is_numeric('\u{202e}'), false);
        assert_eq!(is_numeric('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_numeric('\u{10000}'), false);
        assert_eq!(is_numeric('\u{10001}'), false);

        assert_eq!(is_numeric('\u{20000}'), false);
        assert_eq!(is_numeric('\u{30000}'), false);
        assert_eq!(is_numeric('\u{40000}'), false);
        assert_eq!(is_numeric('\u{50000}'), false);
        assert_eq!(is_numeric('\u{60000}'), false);
        assert_eq!(is_numeric('\u{70000}'), false);
        assert_eq!(is_numeric('\u{80000}'), false);
        assert_eq!(is_numeric('\u{90000}'), false);
        assert_eq!(is_numeric('\u{a0000}'), false);
        assert_eq!(is_numeric('\u{b0000}'), false);
        assert_eq!(is_numeric('\u{c0000}'), false);
        assert_eq!(is_numeric('\u{d0000}'), false);
        assert_eq!(is_numeric('\u{e0000}'), false);

        assert_eq!(is_numeric('\u{efffe}'), false);
        assert_eq!(is_numeric('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_numeric('\u{f0000}'), false);
        assert_eq!(is_numeric('\u{f0001}'), false);
        assert_eq!(is_numeric('\u{ffffe}'), false);
        assert_eq!(is_numeric('\u{fffff}'), false);
        assert_eq!(is_numeric('\u{100000}'), false);
        assert_eq!(is_numeric('\u{100001}'), false);
        assert_eq!(is_numeric('\u{10fffe}'), false);
        assert_eq!(is_numeric('\u{10ffff}'), false);
    }
}
