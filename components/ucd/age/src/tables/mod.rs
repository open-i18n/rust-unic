// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


mod age;

use unic_ucd_core::UnicodeVersion;


pub use self::age::Age;


/// The [Unicode version](http://www.unicode.org/versions/) of data
pub const UNICODE_VERSION: UnicodeVersion = include!("unicode_version.rsv");
