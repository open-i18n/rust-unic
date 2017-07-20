mod client;

use std::error::Error;
use std::fs;

use self::client::DownloadPath;

use serde_yaml;

const DOWNLOADS: &'static str = include_str!("../../config/downloads.yaml");

pub fn download(version: &str) -> Result<(), Box<Error>> {
    let download_paths: Vec<DownloadPath> = serde_yaml::from_str(DOWNLOADS)
        .expect("Failed to parse YAML config file");

    println!("Cleaning data directory...");
    for path in download_paths.iter().map(DownloadPath::dest) {
        if path.exists() {
            fs::remove_file(path)
                .unwrap_or_else(|e| panic!("Failed to clean path {:?}: {}", path, e))
        }
    }

    println!("Downloading {} files...", download_paths.len());
    client::download_all(
        download_paths
            .into_iter()
            .map(|path| DownloadPath {
                url: path.url.replace("{}", version),
                dest: path.dest,
            })
    )
}
