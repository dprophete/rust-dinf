use std::path::PathBuf;

#[derive(Eq, PartialEq)]
pub struct FileEntry {
    pub size: u64,
    pub path: PathBuf,
}

impl Ord for FileEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.size.cmp(&self.size)
    }
}

impl PartialOrd for FileEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
