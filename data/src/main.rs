// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! UNIC - Source Data Download Script
//!
//! # Config Format:
//!
//! ```toml
//! [example]
//! version = "10.0.0"
//! base_url = "http://example.com/path/to/{version}/"
//!
//! [example.resources]
//! "some_data.txt" = "some_data.txt"
//!
//! [example.test_resources]
//! "other_data.txt" = "tests/other_data.txt"
//! ```

#[macro_use]
extern crate clap;

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate zip;

#[macro_use]
extern crate serde_derive;

extern crate toml;

mod client;
mod config;
mod unzipper;

use std::fs;

use config::DownloadMap;

fn main() {
    let matches = clap_app!(
        unic_data =>
            (author: "The UNIC Project Developers")
            (about: "Download data files based on sources.toml configuration")
            (@arg sources: ... "Sources to download data from, or all if not set")
    ).get_matches();

    let sources: Vec<_> = matches.values_of("sources").unwrap_or_default().collect();
    let download_maps = config::get_download_maps(&sources);

    println!("Cleaning destination directories...");
    clean_dirs(&download_maps);

    println!("Downloading {} files:", download_maps.len());
    for dmap in download_maps.iter() {
        println!("    {} => {}", &dmap.url, dmap.dest.display());
    }

    client::download_all(&download_maps).expect("Download error");
    unzipper::unzip_all(&download_maps).expect("Unzip error");
}

fn clean_dirs(download_maps: &[DownloadMap]) {
    let dirs = download_maps
        .iter()
        .map(|ref dmap| dmap.dest.parent().expect("Bad destination path"));
    for dir in dirs {
        let _ = fs::remove_dir_all(dir); // ignore errors
        fs::create_dir_all(dir).expect(&format!(
            "Error creating destination directory: {}",
            dir.display()
        ));
    }
}
