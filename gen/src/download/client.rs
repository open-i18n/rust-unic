use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use futures::{Future, Stream};
use futures::future::join_all;
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DownloadPath {
    pub url: String,
    pub dest: PathBuf,
}

pub fn download_all<'a, I>(paths: I) -> Result<(), Box<Error>>
where
    I: Iterator<Item = DownloadPath>,
{
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let jobs = paths.map(move |path| {
        path.url.parse::<Uri>()
            .map(|uri| {
                client.get(uri).and_then(move |res| {
                    fs::create_dir_all(path.dest.parent().unwrap())
                        .expect("Failed to create directory");
                    File::create(path.dest)
                        .map(move |mut file| {
                            res.body().for_each(move |chunk| {
                                file.write_all(&chunk)
                                    .map_err(From::from)
                            })
                        })
                        .expect("Failed to create file")
                })
            })
            .unwrap_or_else(|e| panic!(e.to_string()))
    });

    core.run(join_all(jobs))?;

    Ok(())
}
