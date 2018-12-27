use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use failure::Error;
use cursive::Cursive;
use cursive::views::TextView;
use cursive::views::ListView;
use cursive::views::SelectView;

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

    // for dir in dirs 
    //     println!("{}", dir);
    // }

    let mut files_view = SelectView::new();
    for file in files {
        // println!("{}", file)
        // string_file.push_str(&format!("{}\n", file));
        files_view.add_item(file, 0);
    }

    let mut siv = Cursive::ncurses();

    siv.add_layer(files_view);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}
