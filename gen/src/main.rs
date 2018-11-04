// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate unic_char_range;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate matches;

extern crate itertools;
extern crate regex;

mod source;
mod writer;

/// Validate component target names passed in
fn validate_component_name(name: String) -> Result<(), String> {
    if matches!(
        name.as_str(),
        "ucd" | "normal" | "segment" | "idna" | "emoji"
    ) {
        Ok(())
    } else {
        Err(format!("Invalid component: `{}`", name))
    }
}

fn main() {
    let matches = clap_app!(unic_gen =>
        (author: "The UNIC Project Developers")
        (about: "Parse source data and generate data tables for UNIC crates")
        (@arg components: ...  {validate_component_name} "Components to generate tables for")
    )
    .get_matches();

    let components: Vec<_> = matches
        .values_of("components")
        .unwrap_or_default()
        .collect();

    if components.is_empty() || components.contains(&"ucd") {
        writer::ucd::generate();
    }
    if components.is_empty() || components.contains(&"normal") {
        writer::normal::generate();
    }
    if components.is_empty() || components.contains(&"segment") {
        writer::segment::generate();
    }
    if components.is_empty() || components.contains(&"idna") {
        writer::idna::generate();
    }
    if components.is_empty() || components.contains(&"emoji") {
        writer::emoji::generate();
    }
}
