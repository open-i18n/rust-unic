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

use crate::source::idna::idna_mapping_table::IDNA_MAPPING;
use crate::source::idna::readme::UNICODE_VERSION;

use crate::writer::common::emit_unicode_version;
use crate::writer::utils::tables::ToRangeCharTable;
use crate::writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_idna_mapping(dir);
}

pub fn emit_idna_mapping(dir: &Path) {
    let contents: String = IDNA_MAPPING.map.to_range_char_table(|entry, f| {
        write!(f, "{}", entry.status)?;
        if matches!(
            entry.status,
            "Mapped" | "Deviation" | "DisallowedStd3Mapped"
        ) {
            // TODO(NIGHTLY_RUST): Use str::escape_unicode()
            write!(f, "(\"")?;
            if let Some(ref s) = entry.mapping {
                for ch in s.chars() {
                    write!(f, "{}", ch.escape_unicode())?;
                }
            }
            write!(f, "\")")?;
        }
        Ok(())
    });

    write(dir, "idna_mapping.rsv", &contents);
}
