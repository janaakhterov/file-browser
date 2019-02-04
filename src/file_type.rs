#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum FileType {
    Directory,
    File,
    SymLink,
}
