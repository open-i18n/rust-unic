// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use super::Name;


/// Methods for character name property.
pub trait CharName {
    /// Get `Name` of the character.
    fn name(self) -> Option<Name>;
}

impl CharName for char {
    #[inline]
    fn name(self) -> Option<Name>; {
        //Name::of(self)
    }
}


#[cfg(test)]
mod tests {
    use super::{CharName, Name};

    /*
    #[test]
    fn test_char_name() {
        assert_eq!('\u{0000}'.name(), "XXX");
        assert_eq!('\u{0041}'.name(), "XXX");
        assert_eq!('\u{10ffff}'.name(), "XXX");
    }
    */
}
