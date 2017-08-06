// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod bsearch_range_map;
mod bsearch_set;
mod bsearch_single_map;

use std::fmt;

pub use self::bsearch_range_map::ToRangeBSearchMap;
pub use self::bsearch_set::ToBSearchSet;
pub use self::bsearch_single_map::ToSingleBSearchMap;

#[derive(Debug)]
struct DisplayWrapper<'a, T: 'a, F: 'a>(&'a T, &'a F)
where
    F: Fn(&T, &mut fmt::Formatter) -> fmt::Result;

impl<'a, T, F> fmt::Display for DisplayWrapper<'a, T, F>
where
    F: Fn(&T, &mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.1(self.0, f)
    }
}
