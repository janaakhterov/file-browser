#[macro_use]
extern crate lazy_static;

use crate::dir_view::DirView;
use config::Config;
use cursive::{views::BoxView, Cursive};
use failure::Error;
use std::{env::current_dir, result::Result};

mod entry;
mod dir_view;

lazy_static! {
    static ref SETTINGS: Config = {
        let mut config = Config::new();

        // TODO: Print error, but don't quit app
        match config.merge(config::File::with_name("settings.json")) {
            Ok(_) => {}
            Err(_) => {}
        }

        config
    };
}

fn main() -> Result<(), Error> {
    let mut siv = Cursive::ncurses();
    siv.set_fps(60);

    siv.load_theme_file("styles.toml").unwrap();

    let dirs_view = BoxView::with_full_screen(DirView::from(current_dir()?)?);
    // let dirs_view = BoxView::with_full_screen(MainView::try_from(Path::new("/usr/bin").to_path_buf())?);

    siv.add_fullscreen_layer(dirs_view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}
