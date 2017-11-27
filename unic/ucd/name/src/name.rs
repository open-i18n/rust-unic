// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use core::fmt;


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Name {
    pieces: &'static [&'static str],
}

#[cfg_attr(feature = "clippy", allow(len_without_is_empty))]
impl Name {
    pub fn of(ch: char) -> Option<Name> {
        data::NAMES.find(ch).map(|pieces| Name { pieces })
    }

    /// Length of the name in bytes.
    pub fn len(&self) -> usize {
        // start with spaces
        let mut len = self.pieces.len().saturating_sub(1);
        for piece in self.pieces {
            len += piece.len();
        }
        len
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (first, rest) = self.pieces.split_first().unwrap();
        f.write_str(first)?;
        for piece in rest {
            f.write_str(" ")?;
            f.write_str(piece)?;
        }
        Ok(())
    }
}

mod data {
    use unic_char_property::tables::CharDataTable;
    include!("../tables/name_values.rsd");
    pub const NAMES: CharDataTable<&[&str]> = include!("../tables/name_map.rsv");
}
