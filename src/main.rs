use std::fs::read_dir;
use std::ffi::OsString;
use std::path::Path;
use std::result::Result;

fn main() {
    let mut files = read_dir(&Path::new("/home/daniel"))
        .map_err(|err| println!("{}", err))
        .unwrap()
        .into_iter()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|dir| dir.file_name())
        .map(OsString::into_string)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    // UGH! Can't chain sort! :(
    files.sort();

    for file in files {
        println!("{}", file);
    }
}
