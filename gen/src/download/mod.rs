mod client;

use std::error::Error;
use std::fs;
use std::path::Path;

use download::client::DownloadPath;

use serde_yaml;

const DOWNLOADS: &'static str = include_str!("../../config/downloads.yaml");

pub fn download(version: &str) -> Result<(), Box<Error>> {
    let download_paths: Vec<DownloadPath> = serde_yaml::from_str(DOWNLOADS)
        .expect("Failed to parse YAML config file");
    println!("Downloading {} files...", download_paths.len());

    fs::remove_dir_all(Path::new("data/ucd"))
        .expect("Failed to clean data/ucd directory");

    client::download_all(
        download_paths
            .into_iter()
            .map(|path| DownloadPath {
                url: path.url.replace("{}", version),
                dest: path.dest,
            })
    )
}
