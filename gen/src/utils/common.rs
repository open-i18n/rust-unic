use std::process::Command;
use std::path::Path;
use std::{fs, io};

pub fn cleanup_data<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if path.as_ref().exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir(&path)?;
    Ok(())
}

pub fn cleanup_rsv<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if path.as_ref().exists() {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy() == "rsv" {
                    fs::remove_file(&path)?;
                }
            }
        }
    }

    Ok(())
}

pub fn fetch<S, P>(url: S, destination: P)
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let (url, destination) = (url.as_ref(), destination.as_ref());
    let curl_response = Command::new("curl")
        .arg(url)
        .output()
        .expect("Failed to run curl");
    if curl_response.status.success() {
        let mut text = curl_response.stdout;
        let mut file = fs::File::create(destination).unwrap();
        io::copy(&mut &text[..], &mut file).unwrap();
    } else {
        panic!(
            "curl failed on url {} with response: {:?}",
            url,
            curl_response
        );
    }
}

fn capitalize(str: &str) -> String {
    let mut chars = str.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(str: &str) -> String {
    str.split_whitespace()
        .map(capitalize)
        .fold(String::new(), |acc, ref word| acc + word)
}
