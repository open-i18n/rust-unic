// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::fmt;

use unic_utils::CharDataTable;


type NamePieces = &'static [&'static str];


/// Represents values of the Unicode character property
/// [*Name*](http://www.unicode.org/reports/tr44/#Name).
///
/// TBD.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Name {
    pieces: NamePieces,
}

impl Name {
    /// Find the character *Name* property value.
    pub fn of(ch: char) -> Option<String> {
        pub const TABLE: &'static [(char, NamePieces)] = include!("tables/name_values.rsv");
        *TABLE.find(ch).cloned()
    }

    /// Human-readable description of the Name property value.
    #[inline]
    pub fn display(&self) -> String {
        "XXX".to_owned()
    }
}


impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}


#[cfg(test)]
mod tests {
    use super::Name;

    /*
    #[test]
    fn test_values() {
        // ASCII
        assert_eq!(Name::of('\u{0000}'), "XXX");
        assert_eq!(Name::of('\u{0021}'), "XXX");
        assert_eq!(Name::of('\u{0041}'), "XXX");
        assert_eq!(Name::of('\u{007f}'), "XXX");

        assert_eq!(Name::of('\u{0100}'), "XXX");
        assert_eq!(Name::of('\u{01f5}'), "XXX");
        assert_eq!(Name::of('\u{037e}'), "XXX");
        assert_eq!(Name::of('\u{200c}'), "XXX");

        assert_eq!(Name::of('\u{01f6}'), "XXX");
        assert_eq!(Name::of('\u{01f7}'), "XXX");
        assert_eq!(Name::of('\u{01f9}'), "XXX");

        assert_eq!(Name::of('\u{0860}'), "XXX");
        assert_eq!(Name::of('\u{0866}'), "XXX");
        assert_eq!(Name::of('\u{086a}'), "XXX");

        assert_eq!(Name::of('\u{fffe}'), "XXX");
        assert_eq!(Name::of('\u{ffff}'), "XXX");

        assert_eq!(Name::of('\u{10000}'), "XXX");
        assert_eq!(Name::of('\u{10001}'), "XXX");

        assert_eq!(Name::of('\u{e0100}'), "XXX");
        assert_eq!(Name::of('\u{e0101}'), "XXX");
        assert_eq!(Name::of('\u{e0170}'), "XXX");
        assert_eq!(Name::of('\u{e01ee}'), "XXX");
        assert_eq!(Name::of('\u{e01ef}'), "XXX");

        assert_eq!(Name::of('\u{10000}'), "XXX");

        assert_eq!(Name::of('\u{20000}'), "XXX");

        assert_eq!(Name::of('\u{30000}'), "XXX");
        assert_eq!(Name::of('\u{40000}'), "XXX");
        assert_eq!(Name::of('\u{50000}'), "XXX");
        assert_eq!(Name::of('\u{60000}'), "XXX");
        assert_eq!(Name::of('\u{70000}'), "XXX");
        assert_eq!(Name::of('\u{80000}'), "XXX");
        assert_eq!(Name::of('\u{90000}'), "XXX");
        assert_eq!(Name::of('\u{a0000}'), "XXX");
        assert_eq!(Name::of('\u{b0000}'), "XXX");
        assert_eq!(Name::of('\u{c0000}'), "XXX");
        assert_eq!(Name::of('\u{d0000}'), "XXX");
        assert_eq!(Name::of('\u{e0000}'), "XXX");
        assert_eq!(Name::of('\u{efffd}'), "XXX");

        assert_eq!(Name::of('\u{efffe}'), "XXX");
        assert_eq!(Name::of('\u{effff}'), "XXX");

        // Priavte-Use Area
        assert_eq!(Name::of('\u{f0000}'), "XXX");
        assert_eq!(Name::of('\u{f0001}'), "XXX");
        assert_eq!(Name::of('\u{ffffe}'), "XXX");
        assert_eq!(Name::of('\u{fffff}'), "XXX");
        assert_eq!(Name::of('\u{100000}'), "XXX");
        assert_eq!(Name::of('\u{100001}'), "XXX");
        assert_eq!(Name::of('\u{10fffe}'), "XXX");
        assert_eq!(Name::of('\u{10ffff}'), "XXX");
    }
    */
}
