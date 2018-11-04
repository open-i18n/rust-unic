// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;
use std::fs::File;
use std::io::Write;

use futures::future::join_all;
use futures::{Future, Stream};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

use config::DownloadMap;

/// Concurrently download all files as indicated by `DownloadMap`s.
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
pub fn download_all(download_paths: &[DownloadMap]) -> Result<(), Box<Error>> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let jobs = download_paths.iter().map(move |path| {
        path.url
            .parse::<Uri>()
            .map(|uri| {
                client.get(uri).and_then(move |res| {
                    File::create(&path.dest)
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
