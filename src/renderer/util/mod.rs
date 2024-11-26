use super::model;
use super::texture;

#[macro_export]
macro_rules! res_path {
    ($relative_path:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/res/", $relative_path)
    };
}

pub mod resources;
