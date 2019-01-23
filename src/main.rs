#[macro_use]
extern crate lazy_static;

use crate::{dir_view::DirView, settings::Settings};
use config::Config;
use failure::Error;
use ncurses::*;
use std::{env::current_dir, result::Result};
use crate::settings::ColorValue;
use crate::colors::init_colors;

mod dir_view;
mod entry;
mod settings;
mod colors;

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

// Initialize the hardcoded colors
// For now this is going to be 4 different filetypes
// Directory, file, sym-link, and exec.
// Planning to add full customizability to colors later on.
fn main() -> Result<(), Error> {
    initscr();
    start_color();

    init_colors();
    ui_settings();

    let mut dirs_view = DirView::from(current_dir()?)?;
    dirs_view.draw(stdscr(), LINES(), COLS());

    loop {
        let c = getch();
        if c == b'q' as i32 {
            break;
        } else if c == b'j' as i32 {
            dirs_view.change_selected_by(1);
            dirs_view.draw(stdscr(), LINES(), COLS());
        } else if c == b'k' as i32 {
            dirs_view.change_selected_by(-1);
            dirs_view.draw(stdscr(), LINES(), COLS());
        } else if c == b'h' as i32 {
            if let Some(parent) = dirs_view.path.parent() {
                wclear(stdscr());

                dirs_view = DirView::from(parent.to_path_buf())?;
                dirs_view.draw(stdscr(), LINES(), COLS());
            }
        } else if c == b'l' as i32 {
            wclear(stdscr());

            dirs_view = DirView::from(dirs_view.selected().path)?;
            dirs_view.draw(stdscr(), LINES(), COLS());
        }
    }

    endwin();

    Ok(())
}

fn ui_settings() {
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    noecho();
    cbreak();
    nonl();
    // nodelay(stdscr(), true);
}

fn textbox_settings() {
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    echo();
    nocbreak();
    nl();
    nodelay(stdscr(), false);
}
