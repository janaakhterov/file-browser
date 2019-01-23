use std::{
    cmp::Ordering,
    fs::{FileType, Metadata, Permissions},
    path::PathBuf,
    time::SystemTime,
};

#[derive(Clone)]
pub struct Entry {
    pub path: PathBuf,
    pub metadata: Metadata,
    pub permissions: Permissions,
    pub modified: SystemTime,
    pub filetype: FileType,
    pub filename: String,
    pub color_regular: i16,
    pub color_selected: i16,
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
