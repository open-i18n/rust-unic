// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


mod idna_mapping;

use reader::idna::readme::UNICODE_VERSION;

use writer::utils::clean_dir;
use writer::common::unicode_version;


pub fn generate() {
    let dir = clean_dir("unic/idna/mapping/tables");

    unicode_version::emit(&dir, &UNICODE_VERSION);
    idna_mapping::emit(&dir);
}
