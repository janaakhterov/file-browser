use crate::directory_view::DirectoryView;
use config::Config;
use cursive::{views::BoxView, Cursive};
use failure::Error;
use std::{path::Path, result::Result};

mod color_pair;
mod directory_view;
mod entry;
#[macro_use]
mod macros;

fn main() -> Result<(), Error> {
    let mut config = Config::default();
    let settings = match config.merge(config::File::with_name("settings.json")) {
        Ok(settings) => settings,
        Err(err) => {
            eprintln!("{}", err);
            &mut config
        }
    };

    let mut siv = Cursive::ncurses();

    siv.load_theme_file("styles.toml").unwrap();

    let dirs_view = BoxView::with_full_screen(DirectoryView::from(
        Path::new("/bin"),
        settings,
    )?);

    siv.add_fullscreen_layer(dirs_view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
    Ok(())
}
