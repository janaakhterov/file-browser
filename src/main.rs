use std::fs::read_dir;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::ffi::OsString;
use std::path::Path;
use std::result::Result;

fn main() {
    println!("Hello, world!");
    let dirs = match read_dir(&Path::new("/home/daniel")) {
        Ok(dirs) => dirs,
        Err(_) => return,
    };

    for file in dirs
        .into_iter()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|dir| dir.file_name())
        .map(OsString::into_string)
        .filter(Result::is_ok)
        .map(Result::unwrap) {
        println!("{}", file);
    }
}
