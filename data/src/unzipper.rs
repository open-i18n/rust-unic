// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::{File, create_dir_all};
use std::io::copy;

use zip::ZipArchive;
use zip::result::ZipError;

use config::DownloadMap;

static ZIP_FILE_EXTENSION: &'static str = "zip";

pub fn unzip_all(download_paths: &[DownloadMap]) -> Result<(), ZipError> {
    for path in download_paths {
        match path.dest.extension() {
            None => continue,
            Some(ext) => {
                if ext != ZIP_FILE_EXTENSION { continue; }

                println!("Unzipping {}...", path.dest.display());

                let zip_file = File::open(&path.dest)?;
                let mut archive = ZipArchive::new(zip_file)?;
                let output_dir = path.dest.with_extension("");
                if !output_dir.exists() {
                    create_dir_all(&output_dir)?;
                }

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let mut extract_dest = output_dir.clone();
                    extract_dest.push(file.name());

                    println!("    {} => {}", file.name(), extract_dest.display());

                    // FIXME: create directory instead of copy if file is a directory
                    let mut output = File::create(&extract_dest)?;
                    copy(&mut file, &mut output)?;
                }
            },
        }
    }
    Ok(())
}
