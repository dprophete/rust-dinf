#[derive(Eq, PartialEq)]
pub struct ExtEntry {
    pub size: u64,
    pub ext: String,
}

impl Ord for ExtEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.size.cmp(&self.size)
    }
}

impl PartialOrd for ExtEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
