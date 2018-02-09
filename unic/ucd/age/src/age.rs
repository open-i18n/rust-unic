// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::cmp;

use unic_char_property::{CharProperty, CustomCharProperty, PartialCharProperty};
pub use unic_ucd_version::UnicodeVersion;

/// Represents values of the Unicode character property
/// [*Age*](https://www.unicode.org/reports/tr44/#Age).
///
/// The Age property indicates the age of a character, which is defined based on the first Unicode
/// version in which a particular Unicode character was *assigned* (as *character* or
/// *noncharacter*).
///
/// Note: `Age` type has a reverse ordering compared to `UnicodeVersion`, because a character is
/// *older* (has greater age) of another character if, and only if, it has a older (smaller)
/// `UnicodeVersion` number.
///
/// Unicode versions with character *assignement* always have the Micro (Update) version value
/// of zero (`0`). Therefore, all `UnicodeVersion` values return from `Age` will have their `macro`
/// field as `0`.
///
/// The *earliest* (largest) value for this property is `UnicodeVersion { major: 1, minor: 1, micro:
/// 0 }`, because of the massive changes for the merger of the Unicode Standard with ISO 10646.
///
/// The *latest* (smallest) value for this property is always equal to or greater than
/// `UNICODE_VERSION`. (Only not equal when `UNICODE_VERSION` has non-zero *micro* value.)
///
/// * <https://www.unicode.org/reports/tr44/#Character_Age>
#[derive(Clone, Copy, Eq, PartialEq, Ord, Debug, Hash)]
pub struct Age(UnicodeVersion);

impl CharProperty for Age {
    fn prop_abbr_name() -> &'static str {
        "age"
    }

    fn prop_long_name() -> &'static str {
        "Age"
    }

    fn prop_human_name() -> &'static str {
        "Age"
    }
}

impl PartialCharProperty for Age {
    fn of(ch: char) -> Option<Self> {
        Self::of(ch)
    }
}

impl CustomCharProperty<UnicodeVersion> for Age {
    /// Get numeric value for character property value
    fn actual(&self) -> UnicodeVersion {
        Self::actual(self)
    }
}

mod data {
    use super::UnicodeVersion;
    use unic_char_property::tables::CharDataTable;
    pub const AGE_TABLE: CharDataTable<UnicodeVersion> = include!("../tables/age_values.rsv");
}

impl Age {
    /// Find the character *Age* property value.
    pub fn of(ch: char) -> Option<Age> {
        data::AGE_TABLE.find(ch).map(Age)
    }

    /// Return the `UnicodeVersion` value of the age.
    pub fn actual(&self) -> UnicodeVersion {
        self.0
    }
}

impl cmp::PartialOrd for Age {
    fn partial_cmp(&self, other: &Age) -> Option<cmp::Ordering> {
        match self.0.cmp(&other.0) {
            cmp::Ordering::Greater => Some(cmp::Ordering::Less),
            cmp::Ordering::Less => Some(cmp::Ordering::Greater),
            cmp::Ordering::Equal => Some(cmp::Ordering::Equal),
        }
    }
}

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
    use super::Age;
    use unic_ucd_version::UnicodeVersion;

    #[test]
    fn test_values() {
        // ASCII
        assert_eq!(
            Age::of('\u{0000}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{0021}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{0041}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{007f}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{0100}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{01f5}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{037e}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{200c}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{01f6}'),
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{01f7}'),
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{01f9}'),
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{0860}'),
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{0866}'),
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{086a}'),
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{fffe}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{ffff}'),
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{10000}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{10001}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{e0100}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{e0101}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{e0170}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{e01ee}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{e01ef}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{10000}'),
            Some(Age(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            }))
        );

        assert_eq!(
            Age::of('\u{20000}'),
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 1,
                micro: 0,
            }))
        );

        assert_eq!(Age::of('\u{30000}'), None);
        assert_eq!(Age::of('\u{40000}'), None);
        assert_eq!(Age::of('\u{50000}'), None);
        assert_eq!(Age::of('\u{60000}'), None);
        assert_eq!(Age::of('\u{70000}'), None);
        assert_eq!(Age::of('\u{80000}'), None);
        assert_eq!(Age::of('\u{90000}'), None);
        assert_eq!(Age::of('\u{a0000}'), None);
        assert_eq!(Age::of('\u{b0000}'), None);
        assert_eq!(Age::of('\u{c0000}'), None);
        assert_eq!(Age::of('\u{d0000}'), None);
        assert_eq!(Age::of('\u{e0000}'), None);
        assert_eq!(Age::of('\u{efffd}'), None);

        assert_eq!(
            Age::of('\u{efffe}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{effff}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );

        // Priavte-Use Area
        assert_eq!(
            Age::of('\u{f0000}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{f0001}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{ffffe}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{fffff}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{100000}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{100001}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{10fffe}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert_eq!(
            Age::of('\u{10ffff}'),
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
    }

    #[test]
    fn test_cmp() {
        assert!(Age::of('A') == Age::of('a'));
        assert!(Age::of('A') > Age::of('Ɐ'));
        assert!(Age::of('A') > Age::of('\u{10000}'));
        assert!(Age::of('A') > Age::of('\u{D0000}'));

        assert!(
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })) == Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })) > Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })) > Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })) > Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })) > None
        );

        assert!(
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })) < Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })) == Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })) > Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })) > Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })) > None
        );

        assert!(
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })) < Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })) < Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })) == Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })) > Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })) > None
        );

        assert!(
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })) < Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })) < Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })) < Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })) == Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })) > None
        );

        assert!(
            None < Some(Age(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }))
        );
        assert!(
            None < Some(Age(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            None < Some(Age(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }))
        );
        assert!(
            None < Some(Age(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }))
        );
    }

    #[test]
    fn test_char_trait() {
        use super::CharAge;

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
