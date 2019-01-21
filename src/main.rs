#[macro_use]
extern crate lazy_static;

use crate::{dir_view::DirView, settings::Settings};
use config::Config;
use failure::Error;
use ncurses::*;
use std::{env::current_dir, result::Result};

mod dir_view;
mod entry;
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

    let s = "#0fffff".to_string();
    if s.chars().next().unwrap() == '#' {
        let s = s.chars().next().map(|c| &s[c.len_utf8()..]).unwrap();
        let s1 = i16::from_str_radix(&s[0..2], 16)?;
        let s2 = i16::from_str_radix(&s[2..4], 16)?;
        let s3 = i16::from_str_radix(&s[4..6], 16)?;

        mvaddstr(11, 0, &s1.to_string());
        mvaddstr(12, 0, &s2.to_string());
        mvaddstr(13, 0, &s3.to_string());
        refresh();
    }

    init_pair(1, COLOR_BLUE, COLOR_BLACK);
    init_pair(2, COLOR_WHITE, COLOR_BLACK);
    init_pair(3, COLOR_WHITE, COLOR_BLUE);
    init_pair(4, COLOR_BLACK, COLOR_WHITE);

    let mut dirs_view = DirView::from(current_dir()?)?;

    dirs_view.draw(stdscr(), LINES(), COLS());
    getch();

    Ok(())
}
