use std::process::{Command, ExitStatus};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct Dir {
    root: &'static Path,
    ucd_data: PathBuf,
    ucd_test_data: PathBuf,
    idna_data: PathBuf,
    idna_test_data: PathBuf,
}

impl Dir {
    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn ucd_data(&self) -> &Path {
        &self.ucd_data
    }
    pub fn ucd_test_data(&self) -> &Path {
        &self.ucd_test_data
    }
    pub fn idna_data(&self) -> &Path {
        &self.idna_data
    }
    pub fn idna_test_data(&self) -> &Path {
        &self.idna_test_data
    }
}

impl Default for Dir {
    fn default() -> Dir {
        let root = Path::new("../../../");
        Dir {
            root: root,
            ucd_data: root.join("data/ucd"),
            ucd_test_data: root.join("data/ucd/test"),
            idna_data: root.join("data/idna"),
            idna_test_data: root.join("data/idna/test"),
        }
    }
}

impl Dir {
    pub fn cleanup_data<P>(path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        fs::remove_dir_all(&path)?;
        fs::create_dir(&path)?;
        Ok(())
    }

    pub fn cleanup_rsv<P>(path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy() == "rsv" {
                    fs::remove_file(&path)?;
                }
            }
        }

        Ok(())
    }
}

pub fn fetch(url: &str, destination: &Path) {
    let curl_exit = Command::new("curl")
        .arg("-o")
        .arg(url)
        .arg(destination.to_str().unwrap())
        .spawn()
        .expect("Failed to launch curl")
        .wait()
        .expect("Failed to await curl");
    if !curl_exit.success() {
        panic!("curl failed with exit code {} for url {}", curl_exit, url)
    }
}
