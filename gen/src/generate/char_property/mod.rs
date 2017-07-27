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
