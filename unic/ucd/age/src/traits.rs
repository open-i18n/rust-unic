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
    fn age(self) -> Option<Age>;
}

impl CharAge for char {
    #[inline]
    fn age(self) -> Option<Age> {
        Age::of(self)
    }
}


#[cfg(test)]
mod tests {
    use unic_ucd_core::UnicodeVersion;

    use super::CharAge;

    #[test]
    fn test_char_age() {
        assert_eq!(
            '\u{0000}'.age().unwrap().actual(),
            UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }
        );
        assert_eq!(
            '\u{0041}'.age().unwrap().actual(),
            UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }
        );
        assert_eq!(
            '\u{10ffff}'.age().unwrap().actual(),
            UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }
        );
    }
}
