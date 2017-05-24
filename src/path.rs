use std;
use std::path::{Path, PathBuf};

/// Converts Path into absolute PathBuf. Should be used only for
/// display in messages since it suppresses any errors and falls back
/// to returning the relative path.
pub fn display_absolute_path(path: &Path) -> PathBuf {
    std::env::current_dir().map(|d| {
        d.join(path)
    }).unwrap_or(path.to_path_buf())
}
