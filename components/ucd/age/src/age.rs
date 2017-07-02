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


use std::cmp::Ordering;

use unic_ucd_core::UnicodeVersion;


/// Represents values of the Unicode character property
/// [*Age*](http://www.unicode.org/reports/tr44/#Age).
///
/// The Age property indicates the first version in which a particular Unicode character was
/// assigned.
///
/// * <http://www.unicode.org/reports/tr44/#Character_Age>
#[allow(non_camel_case_types, missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Age {
    Since { major: u32, minor: u32 },
    Unassigned, // Unassigned is older (larger) than any age
}

pub const AGE_TABLE: &'static [(char, char, Age)] = include!("tables/age_values.rsv");

impl Age {
    /// Find the character Age
    pub fn of(ch: char) -> Age {
        bsearch_range_value_table(ch, AGE_TABLE)
    }

    /// Get `UnicodeVersion` of an `Age` value, or `None`
    pub fn to_unicode_version(self) -> Option<UnicodeVersion> {
        match self {
            Age::Since { major, minor } => Some(UnicodeVersion(major, minor, 0)),
            Age::Unassigned => None,
        }
    }

    /// Get `Age` value from `UnicodeVersion`
    pub fn from_unicode_version(univer: UnicodeVersion) -> Option<Age> {
        if univer < UnicodeVersion(1, 1, 0) {
            None
        } else {
            Some(Age::Since {
                major: univer.0,
                minor: univer.1,
            })
        }
    }
}

// TODO: Generic'ize and move to `unic-ucd-utils`
// TODO: Optimize: put Unassigned ranges into the table, then only store (start, age) instead of
// (start, end, age)
fn bsearch_range_value_table(ch: char, r: &'static [(char, char, Age)]) -> Age {
    match r.binary_search_by(|&(lo, hi, _)| if lo <= ch && ch <= hi {
        Ordering::Equal
    } else if hi < ch {
        Ordering::Less
    } else {
        Ordering::Greater
    }) {
        Ok(idx) => {
            let (_, _, cat) = r[idx];
            cat
        }
        Err(_) => Age::Unassigned,
    }
}

#[cfg(test)]
mod tests {
    use super::Age;
    use unic_ucd_core::UnicodeVersion;

    #[test]
    fn test_age() {
        // ASCII
        assert_eq!(Age::of('\u{0000}'), Age::Since { major: 1, minor: 1 });
        assert_eq!(Age::of('\u{0021}'), Age::Since { major: 1, minor: 1 });
        assert_eq!(Age::of('\u{0041}'), Age::Since { major: 1, minor: 1 });
        assert_eq!(Age::of('\u{007f}'), Age::Since { major: 1, minor: 1 });

        assert_eq!(Age::of('\u{0100}'), Age::Since { major: 1, minor: 1 });
        assert_eq!(Age::of('\u{01f5}'), Age::Since { major: 1, minor: 1 });
        assert_eq!(Age::of('\u{037e}'), Age::Since { major: 1, minor: 1 }); // start == end
        assert_eq!(Age::of('\u{200c}'), Age::Since { major: 1, minor: 1 });

        assert_eq!(Age::of('\u{01f6}'), Age::Since { major: 3, minor: 0 });
        assert_eq!(Age::of('\u{01f7}'), Age::Since { major: 3, minor: 0 });
        assert_eq!(Age::of('\u{01f9}'), Age::Since { major: 3, minor: 0 });

        assert_eq!(
            Age::of('\u{0860}'),
            Age::Since {
                major: 10,
                minor: 0,
            }
        );
        assert_eq!(
            Age::of('\u{0866}'),
            Age::Since {
                major: 10,
                minor: 0,
            }
        );
        assert_eq!(
            Age::of('\u{086a}'),
            Age::Since {
                major: 10,
                minor: 0,
            }
        );

        assert_eq!(Age::of('\u{fffe}'), Age::Since { major: 1, minor: 1 });
        assert_eq!(Age::of('\u{ffff}'), Age::Since { major: 1, minor: 1 });

        assert_eq!(Age::of('\u{10000}'), Age::Since { major: 4, minor: 0 });
        assert_eq!(Age::of('\u{10001}'), Age::Since { major: 4, minor: 0 });

        assert_eq!(Age::of('\u{e0100}'), Age::Since { major: 4, minor: 0 });
        assert_eq!(Age::of('\u{e0101}'), Age::Since { major: 4, minor: 0 });
        assert_eq!(Age::of('\u{e0170}'), Age::Since { major: 4, minor: 0 });
        assert_eq!(Age::of('\u{e01ee}'), Age::Since { major: 4, minor: 0 });
        assert_eq!(Age::of('\u{e01ef}'), Age::Since { major: 4, minor: 0 });

        assert_eq!(Age::of('\u{10000}'), Age::Since { major: 4, minor: 0 });

        assert_eq!(Age::of('\u{20000}'), Age::Since { major: 3, minor: 1 });

        assert_eq!(Age::of('\u{30000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{40000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{50000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{60000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{70000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{80000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{90000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{a0000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{b0000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{c0000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{d0000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{e0000}'), Age::Unassigned);
        assert_eq!(Age::of('\u{efffd}'), Age::Unassigned);

        assert_eq!(Age::of('\u{efffe}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{effff}'), Age::Since { major: 2, minor: 0 });

        // Priavte-Use Area
        assert_eq!(Age::of('\u{f0000}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{f0001}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{ffffe}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{fffff}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{100000}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{100001}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{10fffe}'), Age::Since { major: 2, minor: 0 });
        assert_eq!(Age::of('\u{10ffff}'), Age::Since { major: 2, minor: 0 });
    }

    #[test]
    fn test_age_to_unicode_version() {
        assert_eq!(
            Age::Since { major: 1, minor: 1 }.to_unicode_version(),
            Some(UnicodeVersion(1, 1, 0))
        );
        assert_eq!(Age::Unassigned.to_unicode_version(), None);
    }

    #[test]
    fn test_age_from_unicode_version() {
        assert_eq!(Age::from_unicode_version(UnicodeVersion(0, 2, 0)), None);
        assert_eq!(Age::from_unicode_version(UnicodeVersion(1, 0, 0)), None);
        assert_eq!(
            Age::from_unicode_version(UnicodeVersion(1, 1, 0)),
            Some(Age::Since { major: 1, minor: 1 })
        );
        assert_eq!(
            Age::from_unicode_version(UnicodeVersion(1, 1, 2)),
            Some(Age::Since { major: 1, minor: 1 })
        );
    }

    #[test]
    fn test_age_cmp() {
        assert!(Age::Since { major: 1, minor: 1 } == Age::Since { major: 1, minor: 1 });
        assert!(Age::Since { major: 1, minor: 1 } < Age::Since { major: 2, minor: 0 });
        assert!(Age::Since { major: 1, minor: 1 } < Age::Since { major: 3, minor: 0 });
        assert!(
            Age::Since { major: 1, minor: 1 } <
                Age::Since {
                    major: 10,
                    minor: 0,
                }
        );
        assert!(Age::Since { major: 1, minor: 1 } < Age::Unassigned);

        assert!(Age::Since { major: 2, minor: 0 } > Age::Since { major: 1, minor: 1 });
        assert!(Age::Since { major: 2, minor: 0 } == Age::Since { major: 2, minor: 0 });
        assert!(Age::Since { major: 2, minor: 0 } < Age::Since { major: 3, minor: 0 });
        assert!(
            Age::Since { major: 2, minor: 0 } <
                Age::Since {
                    major: 10,
                    minor: 0,
                }
        );
        assert!(Age::Since { major: 2, minor: 0 } < Age::Unassigned);

        assert!(Age::Since { major: 3, minor: 0 } > Age::Since { major: 1, minor: 1 });
        assert!(Age::Since { major: 3, minor: 0 } > Age::Since { major: 2, minor: 0 });
        assert!(Age::Since { major: 3, minor: 0 } == Age::Since { major: 3, minor: 0 });
        assert!(
            Age::Since { major: 3, minor: 0 } <
                Age::Since {
                    major: 10,
                    minor: 0,
                }
        );
        assert!(Age::Since { major: 3, minor: 0 } < Age::Unassigned);

        assert!(
            Age::Since {
                major: 10,
                minor: 0,
            } > Age::Since { major: 1, minor: 1 }
        );
        assert!(
            Age::Since {
                major: 10,
                minor: 0,
            } > Age::Since { major: 2, minor: 0 }
        );
        assert!(
            Age::Since {
                major: 10,
                minor: 0,
            } > Age::Since { major: 3, minor: 0 }
        );
        assert!(
            Age::Since {
                major: 10,
                minor: 0,
            } ==
                Age::Since {
                    major: 10,
                    minor: 0,
                }
        );
        assert!(
            Age::Since {
                major: 10,
                minor: 0,
            } < Age::Unassigned
        );

        assert!(Age::Unassigned > Age::Since { major: 1, minor: 1 });
        assert!(Age::Unassigned > Age::Since { major: 2, minor: 0 });
        assert!(Age::Unassigned > Age::Since { major: 3, minor: 0 });
        assert!(
            Age::Unassigned >
                Age::Since {
                    major: 10,
                    minor: 0,
                }
        );
        assert!(Age::Unassigned == Age::Unassigned);
    }
}
