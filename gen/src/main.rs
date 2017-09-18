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

extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate matches;
extern crate regex;


mod download;
mod source;
mod writer;


use std::io::Write;


/// Validate component target names passed in
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
// This signature is enforced by clap
fn validate_component_name(name: String) -> Result<(), String> {
    if matches!(name.as_str(), "idna" | "ucd" | "normal" | "emoji") {
        Ok(())
    } else {
        Err(format!(
            "Valid components are `idna`, `ucd`, and `normal`; you put `{}`",
            name
        ))
    }
}

fn main() {
    let matches = clap_app!(unic_gen =>
        (author: "The UNIC Project Developers")
        (about: "Download data files and generate data tables for UNIC crates")
        (@arg components: * ...
            {validate_component_name} "Components to download data and generate tables for")
        (@arg download: -d --download "Download the data files")
        (@arg generate: -g --generate "Generate the data tables")
    ).get_matches();

    let components: Vec<_> = matches
        .values_of("components")
        .expect("Required argument missing")
        .collect();
    let download = matches.is_present("download");
    let generate = matches.is_present("generate");

    if download {
        download::download(&components);
    }

    if generate {
        if components.contains(&"ucd") {
            writer::ucd::generate();
        }
        if components.contains(&"normal") {
            writer::normal::generate();
        }
        if components.contains(&"idna") {
            writer::idna::generate();
        }
    }

    if !download && !generate {
        writeln!(
            std::io::stderr(),
            "{}\n\n\
             Either the --download or --generate flag must be present.\n\
             For more information try --help",
            matches.usage(),
        ).unwrap();
        std::process::exit(1);
    }
}
