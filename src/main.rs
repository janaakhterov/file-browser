#[macro_use]
extern crate lazy_static;

use crate::dir_view::DirView;
use config::Config;
use failure::Error;
use std::{env::current_dir, result::Result};
use crate::settings::Settings;

use ncurses::*;

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
    initscr();
    start_color();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    cbreak();
    nonl();
    // nodelay(stdscr(), true);
    
    let mut dirs_view = DirView::from(current_dir()?)?;

    dirs_view.draw(stdscr(), LINES(), COLS());
    getch();

    Ok(())
}
