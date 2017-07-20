use std::path::Path;

mod update;
mod generate;

pub fn update() {
    update::run();
}

pub fn generate() {
    generate::run();
}

fn ucd_data_dir() -> &'static Path {
    Path::new("data/ucd")
}

fn ucd_test_data_dir() -> &'static Path {
    Path::new("data/ucd/test")
}
