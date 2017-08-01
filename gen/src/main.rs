// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)] // until full integration

#[macro_use]
extern crate clap;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate matches;
extern crate regex;

mod download;
mod generate;

/// Validate component target names passed in
#[cfg_attr(rustfmt, allow(needless_pass_by_value))]
// This signature is enforced by clap
fn validate_component_name(name: String) -> Result<(), String> {
    if matches!(name.as_str(), "idna" | "ucd" | "normal") {
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
        download::download(&components).expect("Failed to download data");
    }

    if generate {
        if components.contains(&"idna") {
            // generate::idna::generate().expect("Failed to generate Idna tables");
            use std::io::Write;
            writeln!(std::io::stderr(), "Use Python generation for idna").unwrap();
        }
        if components.contains(&"ucd") {
            generate::ucd::generate().expect("Failed to generate UCD tables");
        }
        if components.contains(&"normal") {
            generate::normal::generate().expect("Failed to generate normal tables");
        }
    }

    if !download && !generate {
        use std::io::Write;
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
