// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! TBD.


use std::fmt::{Debug, Display};
use std::hash::Hash;


/// TBD.
pub trait CharProperty
where
    Self: Copy + Debug + Default + Display + Eq + Hash,
{
    /// TBD
    fn of(ch: char) -> Self;
}


/// TBD.
pub trait OptionCharProperty
where
    Self: Copy + Debug + Display + Eq + Hash,
{
    /// TBD
    fn of(ch: char) -> Option<Self>;
}


/// TBD.
pub trait EnumeratedCharProperty
where
    Self: Copy + Debug + Display + Eq + Hash,
{
    /// TBD
    fn all_values() -> &'static [Self];
}
