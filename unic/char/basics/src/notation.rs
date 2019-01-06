// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Format Unicode Code-Points in the style recommended by The Unicode Standard.
//!
//! As defined by The Unicode Standard (Appendix A—Notational Conventions), a Unicode code point is
//! referred to by writing "U+" followed by its hexadecimal number. For code points in the Basic
//! Multilingual Plane (BMP), four digits are used; for code points outside the BMP, five or six
//! digits are used, as required.
//!
//! References:
//! - https://www.unicode.org/versions/Unicode10.0.0/appA.pdf

use core::fmt;

/// Represent the Unicode Notation of a code-point.
///
/// - https://www.unicode.org/versions/Unicode10.0.0/appA.pdf
#[derive(Debug)]
pub struct UnicodeNotation {
    codepoint: char,
}

impl fmt::Display for UnicodeNotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U+{:04X}", self.codepoint as u32)
    }
}

/// Return a `UnicodeNotation` for the code-point to be used in string format.
pub fn unicode_notation(codepoint: char) -> UnicodeNotation {
    UnicodeNotation { codepoint }
}
