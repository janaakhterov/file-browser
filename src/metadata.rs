use std::fs;

#[derive(Clone)]
pub enum Metadata {
    LocalHost(fs::Metadata),
    SSH(ssh2::FileStat),
}
