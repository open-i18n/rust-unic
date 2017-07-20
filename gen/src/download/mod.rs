mod client;

use std::error::Error;
use std::fs;
use std::path::Path;

use download::client::DownloadPath;

// TODO: load this in from a config file
lazy_static! {
    static ref DOWNLOADS: Vec<DownloadPath<'static>> = vec![
        // data/ucd
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/DerivedAge.txt".to_owned(),
            dest: Path::new("data/ucd/DerivedAge.txt"),
        },
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/DerivedNormalizationProps.txt".to_owned(),
            dest: Path::new("data/ucd/DerivedNormalizationProps.txt"),
        },
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/ReadMe.txt".to_owned(),
            dest: Path::new("data/ucd/ReadMe.txt"),
        },
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/UnicodeData.txt".to_owned(),
            dest: Path::new("data/ucd/UnicodeData.txt"),
        },
        // data/ucd/test
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/BidiCharacterTest.txt".to_owned(),
            dest: Path::new("data/ucd/test/BidiCharacterTest.txt"),
        },
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/BidiTest.txt".to_owned(),
            dest: Path::new("data/ucd/test/BidiTest.txt"),
        },
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/extracted/DerivedDecompositionType.txt".to_owned(),
            dest: Path::new("data/ucd/test/DecompositionTypeTest.txt"),
        },
        DownloadPath {
            url: "http://www.unicode.org/Public/{}/ucd/NormalizationTest.txt".to_owned(),
            dest: Path::new("data/ucd/test/NormalizationTest.txt"),
        },
    ];
}

pub fn download(version: &str) -> Result<(), Box<Error>> {
    fs::remove_dir_all(Path::new("data/ucd"))
        .expect("Failed to clean data/ucd directory");
    client::download_all(
        DOWNLOADS
            .iter()
            .map(|path| DownloadPath {
                url: path.url.replace("{}", version),
                dest: path.dest,
            })
    )
}
