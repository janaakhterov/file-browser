use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use failure::Error;
use cursive::Cursive;
use cursive::views::SelectView;
use cursive::views::ScrollView;
use cursive::views::IdView;

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

    let files_view = ScrollView::new(IdView::new("files", SelectView::new().with_all_str(files.into_iter())));

    let mut siv = Cursive::ncurses();

    siv.load_theme_file("styles.toml").unwrap();

    siv.add_layer(files_view);

    siv.add_global_callback('q', |s| s.quit());

    siv.add_global_callback('j', |s| {
        s.call_on_id("files", |view: &mut SelectView| {
            view.select_down(1);
        });
    });

    siv.add_global_callback('k', |s| {
        s.call_on_id("files", |view: &mut SelectView| {
            view.select_up(1);
        });
    });

    siv.run();

    Ok(())
}
