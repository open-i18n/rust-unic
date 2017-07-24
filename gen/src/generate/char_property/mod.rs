mod bsearch_map;
mod bsearch_set;

use std::fmt::Display;

pub trait CharMap<T: Eq> : bsearch_map::ToBSearchMap<T> {
    fn to_bsearch_map<F, D>(&self, display_fn: F) -> String
    where
        F: Fn(&T) -> D,
        D: Display,
    {
        bsearch_map::ToBSearchMap::to_bsearch_map(self, display_fn)
    }

    fn to_bsearch_map_default(&self) -> String
    where
        for<'a> &'a T: Display,
    {
        bsearch_map::ToBSearchMap::to_bsearch_map_default(self)
    }
}

impl<T: Eq, M> CharMap<T> for M where M: bsearch_map::ToBSearchMap<T> {}

pub trait CharSet : bsearch_set::ToBSearchSet {
    fn to_bsearch_set(&self) -> String {
        bsearch_set::ToBSearchSet::to_bsearch_set(self)
    }
}

impl<S> CharSet for S where S: bsearch_set::ToBSearchSet {}
