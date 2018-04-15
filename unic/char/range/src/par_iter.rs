extern crate rayon;

use rayon::prelude::*;
use CharRange;

impl IntoParallelIterator for CharRange {
    fn into_par_iter(self) -> Self::Iter {
        compile_error!("")
    }
}
