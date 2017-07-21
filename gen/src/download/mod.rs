mod client;

use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

use self::client::DownloadPath;

use serde_yaml;

/// Path to mapping of files to download (YAML).
///
/// Format:
///
/// ```yaml
/// ---
/// -
///   url: "http://unicode.com/Public/{}/File.txt"
///   dest: data/File.txt
/// -
///   url: "http://unicode.com/Public/{}/DerivedFile.txt"
///   dest: data/FileTest.txt
/// ```
const DOWNLOADS: &'static str = "gen/config/downloads.yaml";

/// Downloads all files (indicated by `config/downloads.yaml`) for the indicated version.
///
/// Downloading the correct version is done via a naive string replacement from `{}` to the
/// provided version string. Files are then saved in the indicated target location.
///
/// # Panics
///
/// Panics if the config file is missing or badly formatted, or if it fails to open a file.
///
/// # Errors
///
/// Returns a boxed error when something goes wrong during download or a write error occurs.
pub fn download(version: &str) -> Result<(), Box<Error>> {
    let file = File::open(Path::new(DOWNLOADS)).expect("Failed to open downloads.yaml");

    let download_paths: Vec<DownloadPath> =
        serde_yaml::from_reader(BufReader::new(file)).expect("Failed to parse downloads.yaml");

    println!("Cleaning data directory...");
    for path in download_paths.iter().map(DownloadPath::dest) {
        if path.exists() {
            fs::remove_file(path)
                .unwrap_or_else(|e| panic!("Failed to clean path {:?}: {}", path, e))
        }
    }

    println!("Downloading {} files...", download_paths.len());
    client::download_all(download_paths.into_iter().map(|path| {
        DownloadPath {
            url: path.url.replace("{}", version),
            dest: path.dest,
        }
    }))
}
