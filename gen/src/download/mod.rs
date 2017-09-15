// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


//! UNIC - Data Download Script
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


mod client;

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::fs;

use toml;

use self::client::DownloadPath;


type Config = HashMap<String, DataSource>;

#[derive(Debug, Deserialize)]
struct DataSource {
    version: String,
    base_url: String,
    resources: HashMap<String, String>,
    test_resources: HashMap<String, String>,
}


/// Downloads all files from the config file.
///
/// # Panics
///
/// Panics if the config file is badly formatted.
///
/// # Errors
///
/// Returns a boxed error when something goes wrong during download or a write error occurs.
pub fn download(active_sources: &Vec<&str>) -> Result<(), Box<Error>> {
    let config: Config =
        toml::from_str(include_str!("config.toml")).expect("Failed to parse config file");

    let mut downloads = vec![];

    for (source, info) in config {
        if !active_sources.contains(&source.as_str()) {
            continue;
        }
        for (name, url_path) in &info.resources {
            let dest: PathBuf = ["data", &source, &name].iter().collect();
            let url = format!("{}{}", info.base_url, url_path).replace("{version}", &info.version);
            downloads.push(DownloadPath { url, dest });
        }
        for (name, url_path) in &info.test_resources {
            let dest: PathBuf = ["data", &source, "test", &name].iter().collect();
            let url = format!("{}{}", info.base_url, url_path).replace("{version}", &info.version);
            downloads.push(DownloadPath { url, dest });
        }
    }

    println!("Cleaning destination directories...");
    for path in downloads.iter().map(DownloadPath::dest) {
        let _ = fs::remove_file(path);  // ignore errors
        fs::create_dir_all(path.parent().unwrap())?;
    }

    println!("Downloading {} files...", downloads.len());
    client::download_all(downloads.into_iter())?;

    Ok(())
}
