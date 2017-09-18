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


//! Unicode *Bidi_Control* Character Property.


use std::convert;
use std::fmt;


mod data {
    use unic_utils::CharDataTable;
    pub const BIDI_CONTROL_TABLE: CharDataTable<()> = include!("../tables/bidi_control.rsv");
}


/// Return whether the given character gets control in Right-to-Left text (`Bidi_Control`).
pub fn is_bidi_control(ch: char) -> bool {
    BidiControl::of(ch).into()
}


use unic_char_property::{BinaryCharProperty, CharProperty, TotalCharProperty};


/// Represents values of the Unicode character property
/// [*Bidi_Control*](http://www.unicode.org/reports/tr44/#Bidi_Control).
///
/// The value is `true` if the character is a "bidi control character", `false` otherwise.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BidiControl(bool);


impl BidiControl {
    /// Boolean value of this character property.
    pub fn is_control(&self) -> bool {
        self.0
    }
}


impl CharProperty for BidiControl {
    fn prop_abbr_name() -> &'static str {
        "Bidi_C"
    }

    fn prop_long_name() -> &'static str {
        "Bidi_Control"
    }

    fn prop_human_name() -> &'static str {
        "Bidi Control"
    }
}


impl TotalCharProperty for BidiControl {
    fn of(ch: char) -> Self {
        Self::of(ch)
    }
}


impl BinaryCharProperty for BidiControl {
    #[inline]
    fn bool(&self) -> bool {
        self.is_control()
    }
}


impl convert::From<BidiControl> for bool {
    fn from(bidi_m: BidiControl) -> bool {
        bidi_m.bool()
    }
}

impl Default for BidiControl {
    #[inline]
    fn default() -> Self {
        BidiControl(false)
    }
}


// NOTE: This cannot be generalized at the moment and needs to be implemented for concrete types
// individually. See <https://users.rust-lang.org/t/12884>.
impl fmt::Display for BidiControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.human_name())
    }
}


impl BidiControl {
    /// Find the character *BidiControl* property value.
    pub fn of(ch: char) -> Self {
        BidiControl(data::BIDI_CONTROL_TABLE.contains(ch))
    }
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

    #[test]
    fn test_display() {
        use super::BidiControl;
        use unic_char_property::BinaryCharProperty;

        assert_eq!(BidiControl::of('\u{200d}').abbr_name(), "N");
        assert_eq!(BidiControl::of('\u{200e}').abbr_name(), "Y");
        assert_eq!(BidiControl::of('\u{200f}').long_name(), "Yes");
        assert_eq!(BidiControl::of('\u{2010}').long_name(), "No");
    }

    #[test]
    fn test_convert_to_bool() {
        use super::BidiControl;

        if BidiControl::of('\u{200e}').into() {
            assert!(true);
        }
    }
}
