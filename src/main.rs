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
use cursive::theme::ColorStyle;
use cursive::utils::span::SpannedString;
use cursive::theme::Style;

fn main() -> Result<(), Error> {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut entries:Vec<(SpannedString<Style>, &'static str)> = Vec::new();

    for entry in read_dir(&Path::new("/home/daniel"))?
        .into_iter()
        .filter(Result::is_ok)
        .map(Result::unwrap) {
            let meta = entry.metadata()?;

            let file_name = entry.file_name().into_string().map_err(|err| panic!(err)).unwrap();

            if meta.is_dir() {
                let file_size = read_dir(&Path::new(&entry.path()))?
                                         .into_iter()
                                         .filter(Result::is_ok)
                                         .map(Result::unwrap)
                                         .collect::<Vec<_>>()
                                         .len();
                let file_size = format!("{:>width$}", file_size, width = 10);
                let file_name = format!("{:<width$}{}", file_name, file_size, width = 30);
                dirs.push(file_name);
            }else if meta.is_file() {
                let file_size = meta.len();
                let file_size = format!("{} B", file_size);
                let file_size = format!("{:>width$}", file_size, width = 10);
                let file_name = format!("{:<width$}{}", file_name, file_size, width = 30);
                files.push(file_name);
            }
    }

    dirs.sort();
    files.sort();

    let mut siv = Cursive::ncurses();

    siv.load_theme_file("styles.toml").unwrap();

    let palette = &siv.current_theme().palette;

    let dirs = dirs.into_iter().map(|dir| {
        (SpannedString::styled(dir, ColorStyle::new(palette.custom("directory").unwrap().clone(), 
                                                    palette.custom("directory-background").unwrap().clone())), "")
    }).collect::<Vec<(SpannedString<Style>, &'static str)>>();

    let files = files.into_iter().map(|file| {
        (SpannedString::styled(file, ColorStyle::new(palette.custom("file").unwrap().clone(), 
                                                    palette.custom("file-background").unwrap().clone())), "")
    }).collect::<Vec<(SpannedString<Style>, &'static str)>>();

    entries.extend_from_slice(dirs.as_slice());
    entries.extend_from_slice(files.as_slice());

    let files_view = ScrollView::new(SelectView::new().with_all(entries.into_iter())).show_scrollbars(false);

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
