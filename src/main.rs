use std::fs::read_dir;
use std::fs::ReadDir;
use std::path::Path;
use std::result::Result;

fn main() {
    println!("Hello, world!");
    let dirs = match read_dir(&Path::new("/home/daniel")) {
        Ok(dirs) => dirs,
        Err(_) => return,
    };

    for dir in dirs.into_iter() {
        match dir {
            Ok(file) => {
                match file.file_name().into_string() {
                    Ok(file_name) => println!("{}", file_name),
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }
    }
}
