use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::Metadata;
use std::ffi::OsString;
use std::path::Path;
use std::result::Result;
use failure::Error;

fn main() -> Result<(), Error> {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for entry in read_dir(&Path::new("/home/daniel/Code/builder_derive"))?
        .into_iter()
        .filter(Result::is_ok)
        .map(Result::unwrap) {
            let meta = entry.metadata()?;
            if meta.is_dir() {
                dirs.push(entry.file_name().into_string().map_err(|err| panic!(err)).unwrap());
            }else if meta.is_file() {
                files.push(entry.file_name().into_string().map_err(|err| panic!(err)).unwrap());
            }
    }

    dirs.sort();
    files.sort();

    for dir in dirs {
        println!("{}", dir);
    }

    for file in files {
        println!("{}", file);
    }

    Ok(())
}
