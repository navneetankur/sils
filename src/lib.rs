#![allow(clippy::needless_return)]
pub mod fs {
use std::path::Path;

/// read_dir recursively.
/// will list files in top dir first.
/// then folders in depth first order.
pub fn walk_dir(path: impl AsRef<Path>) -> WalkDir {
    fn walk_dir_inner(path: &Path) -> WalkDir {
        WalkDir { read_dir: path.read_dir().unwrap(), dirs: Vec::new() }
    }
    return walk_dir_inner(path.as_ref());
}

pub struct WalkDir {
    /// current dir iterator
    read_dir: std::fs::ReadDir,
    /// stack of dir to read when current dir finishes.
    dirs: Vec<std::path::PathBuf>,
}
impl Iterator for WalkDir {
    type Item = std::path::PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        for entry in &mut self.read_dir {
            let path = entry.unwrap().path();
            if path.is_dir() {
                self.dirs.push(path);
            }
            else {
                return Some(path);
            }
        }

        let entry = self.dirs.pop()?;
        self.read_dir = entry.read_dir().unwrap();
        return Some(entry);
    }
}
}
