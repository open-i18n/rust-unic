use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use futures::{Future, Stream};
use futures::future::join_all;
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

/// A mapping between a server resource and a local location.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DownloadPath {
    /// A `http://` string indicating the location of the resource
    pub url: String,
    /// The path to the location where the file should be saved
    pub dest: PathBuf,
}

impl DownloadPath {
    pub fn dest(&self) -> &Path {
        &self.dest
    }
}

/// Concurrently download all files as indicated by the iterator of `DownloadPath`s.
///
/// Returns once all jobs are finished.
///
/// # Panics
///
/// If a file that is supposed to be written cannot be created and opened
///
/// # Errors
///
/// If a error occurs while downloading or writing the files
pub fn download_all<I>(paths: I) -> Result<(), Box<Error>>
where
    I: Iterator<Item = DownloadPath>,
{
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let jobs = paths.map(move |path| {
        path.url
            .parse::<Uri>()
            .map(|uri| {
                client.get(uri).and_then(move |res| {
                    fs::create_dir_all(path.dest.parent().unwrap())
                        .expect("Failed to create directory");
                    File::create(path.dest)
                        .map(move |mut file| {
                            res.body()
                                .for_each(move |chunk| file.write_all(&chunk).map_err(From::from))
                        })
                        .expect("Failed to create file")
                })
            })
            .expect("Invalid URI")
    });

    core.run(join_all(jobs))?;

    Ok(())
}
