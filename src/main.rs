#[macro_use]
extern crate lazy_static;

use crate::{dir_view::DirView, settings::Settings};
use config::Config;
use failure::Error;
use ncurses::*;
use std::{env::current_dir, result::Result};
use crate::settings::ColorValue;
use crate::colors::init_colors;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

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

    let mut history: HashMap<PathBuf, DirView> = HashMap::new();

    let mut path = current_dir()?;
    history.insert(path.clone(), DirView::from(path.clone())?);
    history.get_mut(&path).unwrap().draw(stdscr(), LINES(), COLS());

    loop {
        let c = getch();
        if c == b'q' as i32 {
            break;
        } else if c == b'j' as i32 {
            history.get_mut(&path).unwrap().change_selected_by(1);
            history.get_mut(&path).unwrap().draw(stdscr(), LINES(), COLS());
        } else if c == b'k' as i32 {
            history.get_mut(&path).unwrap().change_selected_by(-1);
            history.get_mut(&path).unwrap().draw(stdscr(), LINES(), COLS());
        } else if c == b'h' as i32 {
            if history.get(&path).unwrap().path.parent().is_some() {
                let parent = history.get(&path).unwrap().path.clone();
                let parent = parent.parent().unwrap();
                path = history.get_mut(&path).unwrap().path.clone();
                wclear(stdscr());

                // if !history.contains_key(&path) {
                //     history.insert(path.clone(), dirs_view.clone());
                // }

                &mut history.insert(parent.to_path_buf(), DirView::from(parent.to_path_buf())?);
                path = parent.to_path_buf();
                &mut history.get_mut(&path).unwrap().change_selected_by_path(path.clone());
                history.get_mut(&path).unwrap().draw(stdscr(), LINES(), COLS());
            }
        } 
        else if c == b'l' as i32 {
            wclear(stdscr());

            // if !history.contains_key(&dirs_view.path) {
            //     history.insert(dirs_view.path.clone(), dirs_view.clone());
            // }

            // if history.contains_key(&dirs_view.selected().path) {
            //     dirs_view = history.get(dirs_view.selected().path);
            // } else {
            //     dirs_view = DirView::from(dirs_view.selected().path)?;
            //     dirs_view.draw(stdscr(), LINES(), COLS());
            //     history.insert(dirs_view.path.clone(), dirs_view.clone());
            // }
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
