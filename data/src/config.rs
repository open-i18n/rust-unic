// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.



use std::collections::HashMap;
use std::path::PathBuf;

use toml;


const CONFIG_TOML: &str = include_str!("../sources.toml");


type Config = HashMap<String, DataSource>;

#[derive(Debug, Deserialize)]
struct DataSource {
    version: String,
    base_url: String,
    resources: HashMap<String, String>,
    test_resources: HashMap<String, String>,
}


/// A mapping between a server resource and a local location.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DownloadMap {
    /// A URL string indicating the location of the resource
    pub url: String,

    /// The path to the location where the file should be saved
    pub dest: PathBuf,
}


/// Downloads all files from the config file.
///
/// # Panics
///
/// Panics if the config file is badly formatted.
pub fn get_download_maps(sources: &Vec<&str>) -> Vec<DownloadMap> {
    let mut maps: Vec<DownloadMap> = vec![];

    let config: Config = toml::from_str(CONFIG_TOML).expect("Failed to parse sources.toml file");

    for (source, info) in config {
        if sources.is_empty() || sources.contains(&source.as_str()) {
            for (name, url_path) in &info.resources {
                let dest: PathBuf = ["data", &source, &name].iter().collect();
                let url =
                    format!("{}{}", info.base_url, url_path).replace("{version}", &info.version);
                maps.push(DownloadMap { url, dest });
            }
            for (name, url_path) in &info.test_resources {
                let dest: PathBuf = ["data", &source, "test", &name].iter().collect();
                let url =
                    format!("{}{}", info.base_url, url_path).replace("{version}", &info.version);
                maps.push(DownloadMap { url, dest });
            }
        }
    }

    maps
}
