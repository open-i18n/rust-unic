// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::path::Path;

use reader::common::UnicodeVersion;

use writer::utils::write;


pub fn emit(dir: &Path, unicode_version: &UnicodeVersion) {
    write(
        dir,
        "unicode_version.rsv",
        &format!(
            "UnicodeVersion {{ major: {}, minor: {}, micro: {} }}",
            unicode_version.major,
            unicode_version.minor,
            unicode_version.micro,
        ),
    );
}
