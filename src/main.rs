#[macro_use]
extern crate lazy_static;

use crate::dir_view::DirView;
use config::Config;
use cursive::{views::BoxView, Cursive};
use failure::Error;
use std::{env::current_dir, result::Result};
use crate::settings::Settings;

mod entry;
mod dir_view;
mod settings;

lazy_static! {
    static ref SETTINGS: Settings = {
        let mut config = Config::new();

        // TODO: Print error, but don't quit app
        match config.merge(config::File::with_name("settings.json")) {
            Ok(_) => {}
            Err(_) => {}
        }

        match config.try_into::<Settings>() {
            Ok(v) => v,
            Err(_) => Settings::default(),
        }
    };
}

fn main() -> Result<(), Error> {
    let mut siv = Cursive::ncurses();
    siv.set_fps(60);

    siv.load_theme_file("styles.toml").unwrap();

    let dirs_view = BoxView::with_full_screen(DirView::from(current_dir()?)?);

    siv.add_fullscreen_layer(dirs_view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}
