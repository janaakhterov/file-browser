use crate::directory_view::DirectoryView;
use config::Config;
use cursive::event::Event;
use cursive::event::Key;
use cursive::theme::ColorStyle;
use cursive::theme::Style;
use cursive::utils::span::SpannedString;
use cursive::views::BoxView;
use cursive::views::IdView;
use cursive::views::ScrollView;
use cursive::views::SelectView;
use cursive::Cursive;
use failure::Error;
use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use std::rc::Rc;

mod directory_view;
mod palette;
#[macro_use]
mod macros;

fn main() -> Result<(), Error> {
    let mut settings = Config::default();

    let settings = match settings.merge(config::File::with_name("settings.json")) {
        Ok(settings) => Rc::new(settings),
        Err(err) => {
            eprintln!("{}", err);
            Rc::new(&mut settings)
        },
    };

    let mut siv = Cursive::ncurses();

    siv.load_theme_file("styles.toml").unwrap();

    let mut dirs_view = BoxView::with_full_screen(DirectoryView::from(Path::new("/home/daniel"))?);

    siv.add_fullscreen_layer(dirs_view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
    Ok(())
}
