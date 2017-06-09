// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use super::{bidi_class, BidiClass, is_explicit, is_rtl};


pub trait BidiChar {
    fn bidi_class(self) -> BidiClass;
    fn is_explicit(self) -> bool;
    fn is_ltr(self) -> bool;
    fn is_rtl(self) -> bool;
}

impl BidiChar for char {
    #[inline]
    fn bidi_class(self) -> BidiClass {
        bidi_class(self)
    }

    #[inline]
    fn is_explicit(self) -> bool {
        is_explicit(bidi_class(self))
    }

    #[inline]
    fn is_ltr(self) -> bool {
        !is_rtl(bidi_class(self))
    }

    #[inline]
    fn is_rtl(self) -> bool {
        is_rtl(bidi_class(self))
    }
}


pub trait BidiStr {
    fn has_explicit(&self) -> bool;
    fn has_ltr(&self) -> bool;
    fn has_rtl(&self) -> bool;
}

impl BidiStr for str {
    #[inline]
    fn has_explicit(&self) -> bool {
        self.chars().any(|ch| ch.is_explicit())
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
        use super::{BidiChar, BidiClass};

        let ch = '\u{0041}'; // U+0041 LATIN CAPITAL LETTER A "A"
        assert_eq!(ch.bidi_class(), BidiClass::L);
        assert!(ch.is_ltr());
        assert!(!ch.is_rtl());

        let ch = '\u{05D0}'; // U+05D0 HEBREW LETTER ALEF "א"
        assert_eq!(ch.bidi_class(), BidiClass::R);
        assert!(!ch.is_ltr());
        assert!(ch.is_rtl());

        let ch = '\u{0627}'; // U+0627 ARABIC LETTER ALEF "ا"
        assert_eq!(ch.bidi_class(), BidiClass::AL);
        assert!(!ch.is_ltr());
        assert!(ch.is_rtl());
    }

    #[test]
    fn test_bidi_str() {
        use super::BidiStr;

        let text = "";
        assert!(!text.has_explicit());
        assert!(!text.has_ltr());
        assert!(!text.has_rtl());

        let text = "\u{0041}\u{05D0}\u{0627}";
        assert!(!text.has_explicit());
        assert!(text.has_ltr());
        assert!(text.has_rtl());
    }
}
