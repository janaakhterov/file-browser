use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use failure::Error;
use cursive::Cursive;
use cursive::views::TextView;

fn main() -> Result<(), Error> {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();

    for entry in read_dir(&Path::new("/home/daniel"))?
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

    // for dir in dirs {
    //     println!("{}", dir);
    // }

    // for file in files {
    //     println!("{}", file);
    // }

    let mut siv = Cursive::ncurses();

    siv.add_layer(TextView::new("Hello World"));

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}
