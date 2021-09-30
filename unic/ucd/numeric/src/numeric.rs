// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use unic_char_property::PartialCharProperty;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NumericValue {
    /// A decimal digit in the range `0..=9`, corresponding to `Numeric_Type=Decimal`
    Decimal(u8),
    /// A digit in the range `0..=9`, corresponding to `Numeric_Type=Digit`
    Digit(u8),
    /// A string representing an integer or a rational number, corresponding to `Numeric_Type=Numeric`
    Numeric(&'static str),
}

mod data {
    use unic_char_property::tables::CharDataTable;
    pub const NUMERIC_VALUES: CharDataTable<u32> = include!("../tables/numeric_values.rsv");
    pub const NUMERIC_STRS: &'static str = include!("../tables/numeric_strs.rsv");
}

impl NumericValue {
    pub fn of(ch: char) -> Option<NumericValue> {
        data::NUMERIC_VALUES.find(ch).map(|i| {
            if (i >> 31) == 0 {
                let idx = (i & 0xffff) as usize;
                let len = (i >> 16) as usize;
                // SAFETY: these bounds are always valid, as they're generated based on
                //         NUMERIC_STRS itself in gen/src/writer/ucd/numeric.rs
                let s = unsafe { data::NUMERIC_STRS.get_unchecked(idx..).get_unchecked(..len) };
                NumericValue::Numeric(s)
            } else {
                let is_decimal = i & (1 << 30) != 0;
                let d = (i & 0xff) as u8;
                if is_decimal {
                    NumericValue::Decimal(d)
                } else {
                    NumericValue::Digit(d)
                }
            }
        })
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            NumericValue::Decimal(d) | NumericValue::Digit(d) => {
                let digits = "0123456789";
                // SAFETY: d is always in range 0..=9, both according to the
                //         spec and verified in gen/src/writer/ucd/numeric.rs
                unsafe { digits.get_unchecked(d as usize..).get_unchecked(..1) }
            }
            NumericValue::Numeric(s) => s,
        }
    }
}

impl PartialCharProperty for NumericValue {
    fn of(ch: char) -> Option<NumericValue> {
        Self::of(ch)
    }
}

impl fmt::Display for NumericValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(NumericValue::of('7').unwrap(), NumericValue::Decimal(7));
        // ARABIC-INDIC DIGIT THREE
        assert_eq!(NumericValue::of('٣').unwrap(), NumericValue::Decimal(3));
    }

    #[test]
    fn test_digit() {
        // CIRCLED DIGIT EIGHT
        assert_eq!(NumericValue::of('⑧').unwrap(), NumericValue::Digit(8));
        // DIGIT THREE FULL STOP
        assert_eq!(NumericValue::of('⒊').unwrap(), NumericValue::Digit(3));
    }

    #[test]
    fn test_numeric() {
        // VULGAR FRACTION ONE HALF
        assert_eq!(NumericValue::of('½').unwrap(), NumericValue::Numeric("1/2"));
        // ROMAN NUMERAL TWELVE
        assert_eq!(NumericValue::of('Ⅻ').unwrap(), NumericValue::Numeric("12"));
    }
}
