use crate::{color_pair::ColorPair, file_type::FileType, metadata::Metadata};
use mime_guess::Mime;
use std::{cmp::Ordering, path::PathBuf};

#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub filename: String,
    pub metadata: Metadata,
    pub filetype: FileType,
    pub mime: Mime,
    pub color: ColorPair,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.filetype.is_dir() && !other.filetype.is_dir() {
            Ordering::Less
        } else if !self.filetype.is_dir() && other.filetype.is_dir() {
            Ordering::Greater
        } else {
            self.filename.cmp(&other.filename)
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.filetype == other.filetype && self.filename == other.filename
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.filetype.is_dir() && !other.filetype.is_dir() {
            Some(Ordering::Less)
        } else if !self.filetype.is_dir() && other.filetype.is_dir() {
            Some(Ordering::Greater)
        } else {
            Some(self.filename.cmp(&other.filename))
        }
    }
}
