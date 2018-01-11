// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! *Alphanumeric* Character Property, equal to `General_Category = Nd | Nl | No or Alphabetic =
//! Yes`.
//!
//! This is equivalent to `Alphabetic = true or Numeric = true`.
//!
//! NOTE: This property is not defined by UCD, but is used commonly enough in Unicode algorithms and
//! applications to provide an optimized implementation.

char_property! {
    /// Represents Unicode characters with `General_Category = Nd | Nl | No`.
    ///
    /// This is equivalent to `Alphabetic = true or Numeric = true`.
    ///
    /// The value is `true` for characters that are alphabetic or have a numeric *General_Category*,
    /// `false` otherwise.
    pub struct Alphanumeric(bool) {
        abbr => "Alphanumeric";
        long => "Alphanumeric";
        human => "Alphanumeric";

        data_table_path => "../tables/alphanumeric.rsv";
    }

    /// Return `true` for alphanumeric characters, `false` otherwise.
    ///
    /// This is equivalent to `is_alphabetic(char) || is_numeric(char)`.
    pub fn is_alphanumeric(char) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_values() {
        use super::is_alphanumeric;

        // ASCII
        assert_eq!(is_alphanumeric('\u{0020}'), false);
        assert_eq!(is_alphanumeric('\u{0021}'), false);
        assert_eq!(is_alphanumeric('\u{0022}'), false);

        assert_eq!(is_alphanumeric('\u{0030}'), true);
        assert_eq!(is_alphanumeric('\u{0031}'), true);
        assert_eq!(is_alphanumeric('\u{0032}'), true);

        assert_eq!(is_alphanumeric('\u{0040}'), false);
        assert_eq!(is_alphanumeric('\u{0041}'), true);
        assert_eq!(is_alphanumeric('\u{0042}'), true);

        assert_eq!(is_alphanumeric('\u{0060}'), false);
        assert_eq!(is_alphanumeric('\u{0061}'), true);
        assert_eq!(is_alphanumeric('\u{0062}'), true);

        assert_eq!(is_alphanumeric('\u{007e}'), false);
        assert_eq!(is_alphanumeric('\u{007f}'), false);

        // Other BMP
        assert_eq!(is_alphanumeric('\u{061b}'), false);
        assert_eq!(is_alphanumeric('\u{061c}'), false);
        assert_eq!(is_alphanumeric('\u{061d}'), false);

        assert_eq!(is_alphanumeric('\u{200d}'), false);
        assert_eq!(is_alphanumeric('\u{200e}'), false);
        assert_eq!(is_alphanumeric('\u{200f}'), false);
        assert_eq!(is_alphanumeric('\u{2010}'), false);

        assert_eq!(is_alphanumeric('\u{2029}'), false);
        assert_eq!(is_alphanumeric('\u{202a}'), false);
        assert_eq!(is_alphanumeric('\u{202e}'), false);
        assert_eq!(is_alphanumeric('\u{202f}'), false);

        // Other Planes
        assert_eq!(is_alphanumeric('\u{10000}'), true);
        assert_eq!(is_alphanumeric('\u{10001}'), true);

        assert_eq!(is_alphanumeric('\u{20000}'), true);
        assert_eq!(is_alphanumeric('\u{30000}'), false);
        assert_eq!(is_alphanumeric('\u{40000}'), false);
        assert_eq!(is_alphanumeric('\u{50000}'), false);
        assert_eq!(is_alphanumeric('\u{60000}'), false);
        assert_eq!(is_alphanumeric('\u{70000}'), false);
        assert_eq!(is_alphanumeric('\u{80000}'), false);
        assert_eq!(is_alphanumeric('\u{90000}'), false);
        assert_eq!(is_alphanumeric('\u{a0000}'), false);
        assert_eq!(is_alphanumeric('\u{b0000}'), false);
        assert_eq!(is_alphanumeric('\u{c0000}'), false);
        assert_eq!(is_alphanumeric('\u{d0000}'), false);
        assert_eq!(is_alphanumeric('\u{e0000}'), false);

        assert_eq!(is_alphanumeric('\u{efffe}'), false);
        assert_eq!(is_alphanumeric('\u{effff}'), false);

        // Priavte-Use Area
        assert_eq!(is_alphanumeric('\u{f0000}'), false);
        assert_eq!(is_alphanumeric('\u{f0001}'), false);
        assert_eq!(is_alphanumeric('\u{ffffe}'), false);
        assert_eq!(is_alphanumeric('\u{fffff}'), false);
        assert_eq!(is_alphanumeric('\u{100000}'), false);
        assert_eq!(is_alphanumeric('\u{100001}'), false);
        assert_eq!(is_alphanumeric('\u{10fffe}'), false);
        assert_eq!(is_alphanumeric('\u{10ffff}'), false);
    }
}
