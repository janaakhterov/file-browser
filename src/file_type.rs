use std::{fs, ops::Not};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum FileType {
    Directory,
    File,
    SymLink,
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        match self {
            FileType::Directory => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileType::File => true,
            _ => false,
        }
    }

    pub fn is_symlink(&self) -> bool {
        match self {
            FileType::SymLink => true,
            _ => false,
        }
    }
}

impl From<ssh2::FileType> for FileType {
    fn from(filetype: ssh2::FileType) -> Self {
        if filetype.is_dir() {
            FileType::Directory
        } else if filetype.is_file() {
            FileType::File
        } else if filetype.is_symlink() {
            FileType::SymLink
        } else {
            panic!("Shouldn't be here");
        }
    }
}

impl From<fs::FileType> for FileType {
    fn from(filetype: fs::FileType) -> Self {
        if filetype.is_dir() {
            FileType::Directory
        } else if filetype.is_file() {
            FileType::File
        } else if filetype.is_symlink() {
            FileType::SymLink
        } else {
            panic!("Shouldn't be here");
        }
    }
}

impl Not for FileType {
    type Output = FileType;

    fn not(self) -> FileType {
        match self {
            FileType::Directory => FileType::File,
            FileType::File => FileType::Directory,
            FileType::SymLink => FileType::Directory,
        }
    }
}
