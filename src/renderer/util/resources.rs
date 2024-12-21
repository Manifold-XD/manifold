use log::info;

use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub fn res_path(relative_file_path: &PathBuf) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("res")
        .join(relative_file_path)
}

pub fn load_resource(relative_file_path: &PathBuf) -> Result<Vec<u8>, Error> {
    let path = res_path(relative_file_path);
    let path_str = path.display();
    info!("Loading resource from path: {path_str}");
    fs::read(&path)
}
