// Copyright 2018 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::Path;

use source::ucd::name_aliases::NAME_ALIASES_DATA;
use source::ucd::readme::UNICODE_VERSION;

use writer::common::emit_unicode_version;
use writer::utils::tables::ToDirectCharTable;
use writer::utils::write;

pub fn generate(dir: &Path) {
    emit_unicode_version(dir, &UNICODE_VERSION);
    emit_name_aliases_table(dir);
}

fn emit_name_aliases_table(dir: &Path) {
    macro_rules! write_map_to_file {
        ($map: ident, $file: expr) => {
            write(
                dir,
                $file,
                &NAME_ALIASES_DATA.$map.to_direct_char_table(|aliases, f| {
                    write!(
                        f,
                        "&[{}]",
                        aliases
                            .iter()
                            .map(|alias| format!("\"{}\"", alias.to_owned()))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }),
            );
        }
    }

    write_map_to_file!(corrections, "corrections.rsv");
    write_map_to_file!(controls, "controls.rsv");
    write_map_to_file!(alternates, "alternates.rsv");
    write_map_to_file!(figments, "figments.rsv");
    write_map_to_file!(abbreviations, "abbreviations.rsv");
}
