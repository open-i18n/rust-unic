// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! *Version* data types.

use core::fmt;

/// Represents a *Unicode Version* type.
///
/// UNIC's *Unicode Version* type is used for Unicode datasets and specifications, including The
/// Unicode Standard (TUS), Unicode Character Database (UCD), Common Local Data Repository (CLDR),
/// IDNA, Emoji, etc.
///
/// Refs:
/// - <https://www.unicode.org/versions/>
/// - <https://www.unicode.org/L2/L2017/17222.htm#152-C3>
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default)]
pub struct UnicodeVersion {
    /// Major version.
    pub major: u8,

    /// Minor version.
    pub minor: u8,

    /// Micro (or Update) version.
    pub micro: u8,
}

impl fmt::Display for UnicodeVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

/// Convert from Rust's internal Unicode Version.
///
/// `std::char::UNICODE_VERSION` is a tuple of the format `(major, minor, macro)`.
impl From<(u8, u8, u8)> for UnicodeVersion {
    fn from((major, minor, micro): (u8, u8, u8)) -> UnicodeVersion {
        UnicodeVersion {
            major,
            minor,
            micro,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::char;

    #[test]
    fn test_against_rust_internal() {
        use super::UnicodeVersion;

        let core_unicode_version: UnicodeVersion = char::UNICODE_VERSION.into();
        assert!(core_unicode_version.major >= 10);
    }
}
