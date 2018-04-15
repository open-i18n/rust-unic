extern crate rayon;

use core::char;
use self::rayon::prelude::*;
use CharRange;

#[derive(Clone, Debug)]
pub struct Iter(rayon::iter::FilterMap<rayon::range::Iter<u32>, fn(u32) -> Option<char>>);

impl ParallelIterator for Iter {
    type Item = char;
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        self.0.drive_unindexed(consumer)
    }
}

impl IntoParallelIterator for CharRange {
    type Iter = Iter;
    type Item = char;
    fn into_par_iter(self) -> Self::Iter {
        Iter(((self.low as u32)..(self.high as u32 + 1)).into_par_iter().filter_map(char::from_u32))
    }
}
