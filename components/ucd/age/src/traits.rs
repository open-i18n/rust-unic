// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use super::Age;


/// Methods for character age property.
pub trait CharAge {
    /// Get `Age` of the character.
    fn age(self) -> Age;
}

impl CharAge for char {
    #[inline]
    fn age(self) -> Age {
        Age::of(self)
    }
}


#[cfg(test)]
mod tests {
    use unic_ucd_core::UnicodeVersion;

    use super::{Age, CharAge};

    #[test]
    fn test_char_age() {
        assert_eq!('\u{0000}'.age(), Age::Assigned(UnicodeVersion(1, 1, 0)));
        assert_eq!('\u{0041}'.age(), Age::Assigned(UnicodeVersion(1, 1, 0)));
        assert_eq!('\u{10ffff}'.age(), Age::Assigned(UnicodeVersion(2, 0, 0)));
    }
}
