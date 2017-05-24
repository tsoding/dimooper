use std;
use std::path::{Path, PathBuf};

pub fn absolute_path(path: &Path) -> PathBuf {
    std::env::current_dir().map(|d| {
        d.join(path)
    }).unwrap_or(path.to_path_buf())
}
