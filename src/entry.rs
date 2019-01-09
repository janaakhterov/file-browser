use crate::color_pair::ColorPair;
use std::cmp::Ordering;

pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) size: String,
    pub(crate) color: ColorPair,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
