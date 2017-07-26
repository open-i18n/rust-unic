mod client;

use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use self::client::DownloadPath;

use toml;

/// The config file
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Config {
    idna: DataSource,
    ucd: DataSource,
}

/// Information under one component heading in the config
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct DataSource {
    version: String,
    resources: Vec<DownloadPath>,
}

/// Path to configuration file.
///
/// Format:
///
/// ```toml
/// [ucd]
/// version = "10.0.0"
///
/// [[ucd.resources]]
/// url = "http://example.com/resources/{version}/data.txt"
/// dest = "data/example/data.txt"
/// ```
const DOWNLOADS: &'static str = "gen/config/config.toml";

/// Downloads all files (indicated by `config/downloads.yaml`) for the indicated version.
///
/// Downloading the correct version is done via a naive string replacement from `{}` to the
/// provided version string. Files are then saved in the indicated target location.
///
/// # Panics
///
/// Panics if the config file is missing or badly formatted, or if it fails to open a file.
/// Panics if an invalid component is passed in.
///
/// # Errors
///
/// Returns a boxed error when something goes wrong during download or a write error occurs.
pub fn download(components: &[&str]) -> Result<(), Box<Error>> {
    let mut file = File::open(Path::new(DOWNLOADS)).expect("Failed to open config.toml");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read config.toml");

    let config: Config = toml::from_str(&buffer).expect("Failed to parse downloads.yaml");

    let mut downloads = vec![];
    for component in components {
        match *component {
            "idna" => {
                let version = &config.idna.version;
                for download in &config.idna.resources {
                    downloads.push(DownloadPath {
                        url: download.url.replace("{version}", version),
                        dest: download.dest.clone(),
                    })
                }
            }
            "ucd" => {
                let version = &config.ucd.version;
                for download in &config.ucd.resources {
                    downloads.push(DownloadPath {
                        url: download.url.replace("{version}", version),
                        dest: download.dest.clone(),
                    })
                }
            }
            _ => panic!("Invalid component to download data for"),
        }
    }

    println!("Cleaning data directories...");
    for path in downloads.iter().map(DownloadPath::dest) {
        let _ = fs::remove_file(path); // ignore errors
        fs::create_dir_all(path.parent().unwrap())?;
    }

    println!("Downloading {} files...", downloads.len());
    client::download_all(downloads.into_iter())
}
