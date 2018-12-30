use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use failure::Error;
use cursive::Cursive;
use cursive::views::SelectView;
use cursive::views::ScrollView;
use cursive::views::IdView;
use cursive::views::BoxView;
use cursive::event::Event;
use cursive::event::Key;

fn main() -> Result<(), Error> {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut entries: Vec<String> = Vec::new();
    let mut test_data: Vec<String> = Vec::new();

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

    for _ in 0..40 {
        test_data.push("test".to_string());
    }

    dirs.sort();
    files.sort();

    entries.extend_from_slice(dirs.as_slice());
    entries.extend_from_slice(files.as_slice());
    entries.extend_from_slice(test_data.as_slice());

    let files_view = ScrollView::new(SelectView::new().with_all_str(entries.into_iter())).show_scrollbars(false);

    let mut siv = Cursive::ncurses();

    siv.load_theme_file("styles.toml").unwrap();

    siv.add_fullscreen_layer(files_view);

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('j', |s| {
        s.on_event(Event::Key(Key::Down));
    });
    siv.add_global_callback('k', |s| {
        s.on_event(Event::Key(Key::Up));
    });

    siv.run();

    Ok(())
}
