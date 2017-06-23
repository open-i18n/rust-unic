// Copyright 2016 The rust-url developers.
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


/// Represents the IDNA Mapping status of the Unicode character.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Mapping {
    /// Valid, and not modified.
    Valid,

    /// Removed from the string.
    Ignored,

    /// Replaced in the string.
    Mapped(StringTableSlice),

    /// Valid if *Nontransitional Processing*, or Mapped if Transitional Processing.
    Deviation(StringTableSlice),

    /// Not allowed, result in error.
    Disallowed,

    /// Disallowed if *UseSTD3ASCIIRules* flag is set, Valid otherwise.
    DisallowedStd3Valid,

    /// Disallowed if *UseSTD3ASCIIRules* flag is set, Mapped otherwise.
    DisallowedStd3Mapped(StringTableSlice),
}

struct Range {
    from: char,
    to: char,
    mapping: Mapping,
}


use self::Mapping::*;

const MAP: &'static [Range] = include!("tables/idna_map.rsv");
const MAP_STRING: &'static str = include!("tables/idna_map_string.rsv");


impl Mapping {
    /// Get Mapping status of the character.
    pub fn of(ch: char) -> &'static Mapping {
        let r = MAP.binary_search_by(|range| if ch > range.to {
            Ordering::Less
        } else if ch < range.from {
            Ordering::Greater
        } else {
            Ordering::Equal
        });
        r.ok().map(|i| &MAP[i].mapping).unwrap()
    }
}


/// String target value of *mapped* mapping statuses.
#[derive(Debug, PartialEq, Eq)]
pub struct StringTableSlice {
    byte_start: u16,
    byte_len: u16,
}

impl StringTableSlice {
    /// Return the string value.
    pub fn value(&self) -> &'static str {
        let start = self.byte_start as usize;
        let len = self.byte_len as usize;
        &MAP_STRING[start..(start + len)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping() {
        use Mapping::*;

        assert_eq!(*Mapping::of('\u{0}'), DisallowedStd3Valid);
        assert_eq!(*Mapping::of('-'), Valid);
        assert_eq!(
            *Mapping::of('A'),
            Mapped(StringTableSlice {
                byte_start: 0,
                byte_len: 1,
            })
        );
        assert_eq!(*Mapping::of('\u{80}'), Disallowed);
        assert_eq!(
            *Mapping::of('\u{a0}'),
            DisallowedStd3Mapped(StringTableSlice {
                byte_start: 26,
                byte_len: 1,
            })
        );
        assert_eq!(*Mapping::of('\u{ad}'), Ignored);
        assert_eq!(
            *Mapping::of('\u{200c}'),
            Deviation(StringTableSlice {
                byte_start: 0,
                byte_len: 0,
            })
        );
    }
}
