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

pub use unic_ucd_core::UnicodeVersion;


/// Represents values of the Unicode character property
/// [*Age*](http://www.unicode.org/reports/tr44/#Age).
///
/// The Age property indicates the first version in which a particular Unicode character was
/// `Assigned`, if the code point is assigned (as character or noncharacter), otherwise
/// `Unassigned`.
///
/// Character *assignement* values always have Unicode Micro (Update) Version value of zero (`0`).
///
/// The *earliest* value for this property is `UnicodeVersion { major: 1, minor: 1, micro: 0 }`,
/// because of the massive changes for the merger of the Unicode Stanrda with ISO 10646.
///
/// The *latest* value for this property is always equal to or less than `UNICODE_VERSION`. (Only
/// not equal when `UNICODE_VERSION` has non-zero *micro* value.)
///
/// * <http://www.unicode.org/reports/tr44/#Character_Age>
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Age {
    /// Assigned Unicode Code Point (as character or noncharacter).
    Assigned(UnicodeVersion),

    /// Unassigned Unicode Code Point (can be assigned in future).
    Unassigned, // Unassigned is older (larger) than any age
}

use Age::{Assigned, Unassigned};

pub const AGE_TABLE: &'static [(char, char, Age)] = include!("tables/age_values.rsv");

impl Age {
    /// Find the character Age
    pub fn of(ch: char) -> Age {
        bsearch_range_value_table(ch, AGE_TABLE)
    }

    /// Return `Some(unicode_version)`, if code point is assigned (as character or noncharacter,
    /// under current `UNICODE_VERSION`), otherwise `None`.
    pub fn assigned(&self) -> Option<UnicodeVersion> {
        match *self {
            Assigned(unicode_version) => Some(unicode_version),
            Unassigned => None,
        }
    }

    /// Return `true` if code point is assigned (as character or noncharacter, under current
    /// `UNICODE_VERSION`), otherwise `false`.
    #[inline]
    pub fn is_assigned(&self) -> bool {
        match *self {
            Assigned(_) => true,
            Unassigned => false,
        }
    }

    /// Return `false` if code point is assigned (as character or noncharacter, under current
    /// `UNICODE_VERSION`), otherwise `true`.
    #[inline]
    pub fn is_unassigned(&self) -> bool {
        match *self {
            Assigned(_) => false,
            Unassigned => true,
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
        Err(_) => Unassigned,
    }
}

#[cfg(test)]
mod tests {
    use super::{Age, Assigned, Unassigned};
    use unic_ucd_core::UnicodeVersion;

    #[test]
    fn test_age_values() {
        // ASCII
        assert_eq!(
            Age::of('\u{0000}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{0021}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{0041}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{007f}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{0100}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{01f5}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{037e}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{200c}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{01f6}'),
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{01f7}'),
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{01f9}'),
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{0860}'),
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{0866}'),
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{086a}'),
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{fffe}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{ffff}'),
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{10000}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{10001}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{e0100}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{e0101}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{e0170}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{e01ee}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{e01ef}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{10000}'),
            Assigned(UnicodeVersion {
                major: 4,
                minor: 0,
                micro: 0,
            })
        );

        assert_eq!(
            Age::of('\u{20000}'),
            Assigned(UnicodeVersion {
                major: 3,
                minor: 1,
                micro: 0,
            })
        );

        assert_eq!(Age::of('\u{30000}'), Unassigned);
        assert_eq!(Age::of('\u{40000}'), Unassigned);
        assert_eq!(Age::of('\u{50000}'), Unassigned);
        assert_eq!(Age::of('\u{60000}'), Unassigned);
        assert_eq!(Age::of('\u{70000}'), Unassigned);
        assert_eq!(Age::of('\u{80000}'), Unassigned);
        assert_eq!(Age::of('\u{90000}'), Unassigned);
        assert_eq!(Age::of('\u{a0000}'), Unassigned);
        assert_eq!(Age::of('\u{b0000}'), Unassigned);
        assert_eq!(Age::of('\u{c0000}'), Unassigned);
        assert_eq!(Age::of('\u{d0000}'), Unassigned);
        assert_eq!(Age::of('\u{e0000}'), Unassigned);
        assert_eq!(Age::of('\u{efffd}'), Unassigned);

        assert_eq!(
            Age::of('\u{efffe}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{effff}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );

        // Priavte-Use Area
        assert_eq!(
            Age::of('\u{f0000}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{f0001}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{ffffe}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{fffff}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{100000}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{100001}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{10fffe}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
        assert_eq!(
            Age::of('\u{10ffff}'),
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            })
        );
    }

    #[test]
    fn test_age_cmp() {
        assert!(
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }) ==
                Assigned(UnicodeVersion {
                    major: 1,
                    minor: 1,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }) <
                Assigned(UnicodeVersion {
                    major: 2,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }) <
                Assigned(UnicodeVersion {
                    major: 3,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }) <
                Assigned(UnicodeVersion {
                    major: 10,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 1,
                minor: 1,
                micro: 0,
            }) < Unassigned
        );

        assert!(
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }) >
                Assigned(UnicodeVersion {
                    major: 1,
                    minor: 1,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }) ==
                Assigned(UnicodeVersion {
                    major: 2,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }) <
                Assigned(UnicodeVersion {
                    major: 3,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }) <
                Assigned(UnicodeVersion {
                    major: 10,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 2,
                minor: 0,
                micro: 0,
            }) < Unassigned
        );

        assert!(
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }) >
                Assigned(UnicodeVersion {
                    major: 1,
                    minor: 1,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }) >
                Assigned(UnicodeVersion {
                    major: 2,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }) ==
                Assigned(UnicodeVersion {
                    major: 3,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }) <
                Assigned(UnicodeVersion {
                    major: 10,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 3,
                minor: 0,
                micro: 0,
            }) < Unassigned
        );

        assert!(
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }) >
                Assigned(UnicodeVersion {
                    major: 1,
                    minor: 1,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }) >
                Assigned(UnicodeVersion {
                    major: 2,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }) >
                Assigned(UnicodeVersion {
                    major: 3,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }) ==
                Assigned(UnicodeVersion {
                    major: 10,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Assigned(UnicodeVersion {
                major: 10,
                minor: 0,
                micro: 0,
            }) < Unassigned
        );

        assert!(
            Unassigned >
                Assigned(UnicodeVersion {
                    major: 1,
                    minor: 1,
                    micro: 0,
                })
        );
        assert!(
            Unassigned >
                Assigned(UnicodeVersion {
                    major: 2,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Unassigned >
                Assigned(UnicodeVersion {
                    major: 3,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(
            Unassigned >
                Assigned(UnicodeVersion {
                    major: 10,
                    minor: 0,
                    micro: 0,
                })
        );
        assert!(Unassigned == Unassigned);
    }
}
