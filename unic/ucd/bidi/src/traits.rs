// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use super::{BidiClass, BidiClassCategory};


/// Methods for bidi properties of character types.
pub trait CharBidiClass {
    /// Get `BidiClass` of the character.
    fn bidi_class(self) -> BidiClass;

    /// Whether the character has *left-to-right* (LTR) bidi directionality.
    fn is_ltr(self) -> bool;

    /// Whether the character has *right-to-left* (RTL) bidi directionality.
    fn is_rtl(self) -> bool;
}

impl CharBidiClass for char {
    #[inline]
    fn bidi_class(self) -> BidiClass {
        BidiClass::of(self)
    }

    #[inline]
    fn is_ltr(self) -> bool {
        BidiClass::of(self).is_ltr()
    }

    #[inline]
    fn is_rtl(self) -> bool {
        BidiClass::of(self).is_rtl()
    }
}


/// Methods for bidi properties of string types.
pub trait StrBidiClass {
    /// Whether the string has any *explicit* bidi formatting character.
    fn has_bidi_explicit(&self) -> bool;

    /// Whether the string has any character with *left-to-right* (LTR) bidi directionality.
    fn has_ltr(&self) -> bool;

    /// Whether the string has any character with *right-to-left* (RTL) bidi directionality.
    fn has_rtl(&self) -> bool;
}

impl StrBidiClass for str {
    #[inline]
    fn has_bidi_explicit(&self) -> bool {
        self.chars().any(|ch| {
            ch.bidi_class().category() == BidiClassCategory::ExplicitFormatting
        })
    }

    #[inline]
    fn has_ltr(&self) -> bool {
        self.chars().any(|ch| ch.is_rtl())
    }

    #[inline]
    fn has_rtl(&self) -> bool {
        self.chars().any(|ch| ch.is_rtl())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_bidi_char() {
        use super::{BidiClass, BidiClassCategory, CharBidiClass};

        let ch = '\u{0041}'; // U+0041 LATIN CAPITAL LETTER A "A"
        assert_eq!(ch.bidi_class(), BidiClass::LeftToRight);
        assert_eq!(ch.bidi_class().category(), BidiClassCategory::Strong);
        assert!(ch.is_ltr());
        assert!(!ch.is_rtl());

        let ch = '\u{05D0}'; // U+05D0 HEBREW LETTER ALEF "א"
        assert_eq!(ch.bidi_class(), BidiClass::RightToLeft);
        assert_eq!(ch.bidi_class().category(), BidiClassCategory::Strong);
        assert!(!ch.is_ltr());
        assert!(ch.is_rtl());

        let ch = '\u{0627}'; // U+0627 ARABIC LETTER ALEF "ا"
        assert_eq!(ch.bidi_class(), BidiClass::ArabicLetter);
        assert_eq!(ch.bidi_class().category(), BidiClassCategory::Strong);
        assert!(!ch.is_ltr());
        assert!(ch.is_rtl());
    }

    #[test]
    fn test_bidi_str() {
        use super::StrBidiClass;

        let text = "";
        assert!(!text.has_bidi_explicit());
        assert!(!text.has_ltr());
        assert!(!text.has_rtl());

        let text = "\u{0041}\u{05D0}\u{0627}";
        assert!(!text.has_bidi_explicit());
        assert!(text.has_ltr());
        assert!(text.has_rtl());
    }
}
