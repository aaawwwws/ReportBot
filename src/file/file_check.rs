use std::fs;
use std::path::Path;
pub struct FileCeck;

impl FileCeck {
    pub fn is_file(path: &str) -> bool {
        return Path::new(path).exists() && fs::metadata(path).unwrap().is_file();
    }
}
