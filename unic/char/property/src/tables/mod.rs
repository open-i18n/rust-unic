// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Character data tables used in UNIC.

mod char_map;
pub use self::char_map::{CharMap, Iter as CharMapIter};

mod char_range_map;
pub use self::char_range_map::{CharRangeMap, Iter as CharRangeMapIter};

// TODO(CAD97): BoolTrie
// TODO(CAD97): PHF ?
// TODO(CAD97): char->Name optimization
// TODO(CAD97): Name->char FST ?
