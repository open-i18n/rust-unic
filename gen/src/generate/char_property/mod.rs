mod bsearch_range_map;
mod bsearch_set;

pub use self::bsearch_range_map::ToRangeBSearchMap;
pub use self::bsearch_set::ToBSearchSet;

use std::fmt::Display;

pub trait CharMap<T: Eq> : ToRangeBSearchMap<T> {
    fn to_range_bsearch_map<F, D>(&self, display_fn: F) -> String
    where
        F: Fn(&T) -> D,
        D: Display,
    {
        ToRangeBSearchMap::to_range_bsearch_map(self, display_fn)
    }

    fn to_range_bsearch_map_default(&self) -> String
    where
        for<'a> &'a T: Display,
    {
        ToRangeBSearchMap::to_range_bsearch_map_default(self)
    }
}

impl<T: Eq, M> CharMap<T> for M where M: ToRangeBSearchMap<T> {}

pub trait CharSet : bsearch_set::ToBSearchSet {
    fn to_bsearch_set(&self) -> String {
        ToBSearchSet::to_bsearch_set(self)
    }
}

impl<S> CharSet for S where S: ToBSearchSet {}
