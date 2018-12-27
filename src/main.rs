use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use failure::Error;
use cursive::Cursive;
use cursive::views::TextView;
use cursive::views::ListView;
use cursive::views::SelectView;
use cursive::views::ScrollView;
use cursive::theme::load_default;

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


    let mut files_view = ScrollView::new(SelectView::new().with_all_str(files.into_iter()));

    let mut siv = Cursive::ncurses();
    let mut theme = siv.current_theme().clone();
    theme.shadow = false;
    siv.set_theme(theme);

    siv.add_layer(files_view);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

    Ok(())
}
