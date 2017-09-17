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


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Name {
    pieces: &'static [&'static str],
}

impl Name {
    pub fn of(ch: char) -> Option<Name> {
        data::NAMES.find(ch).map(|pieces| Name { pieces })
    }

    pub fn to_string(&self) -> String {
        self.pieces.join(" ")
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

mod data {
    use unic_utils::CharDataTable;
    include!("../tables/name_values.rsd");
    pub const NAMES: CharDataTable<&[&str]> = include!("../tables/name_map.rsv");
}


#[cfg(test)]
mod test {
    use super::Name;
    #[test]
    fn basic_tests() {
        assert_eq!(
            Name::of('A').expect("No name for A").to_string(),
            "LATIN CAPITAL LETTER A"
        );
        assert_eq!(Name::of('\u{10FFFF}'), None);

        assert!(Name::of('A') < Name::of('B'));
    }
}
