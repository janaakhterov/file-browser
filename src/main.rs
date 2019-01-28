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
use parking_lot::Mutex;

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

    let mut history: HashMap<PathBuf, Arc<Mutex<DirView>>> = HashMap::new();

    let mut view: Arc<Mutex<DirView>> = Arc::new(Mutex::new(DirView::from(current_dir()?)?));
    history.insert(view.lock().path.clone(), view.clone());
    view.lock().draw(stdscr(), LINES(), COLS());

    loop {
        let c = getch();
        if c == b'q' as i32 {
            break;
        } else if c == b'j' as i32 {
            view.lock().change_selected_by(1);
            view.lock().draw(stdscr(), LINES(), COLS());
        } else if c == b'k' as i32 {
            view.lock().change_selected_by(-1);
            view.lock().draw(stdscr(), LINES(), COLS());
        } else if c == b'h' as i32 {
            let parent = view.lock().path.clone();
            let parent = parent.parent();
            if parent.is_some() {
                wclear(stdscr());

                let parent = parent.unwrap().to_path_buf();
                view = if history.get(&parent).is_some() {
                    history.get(&parent).unwrap().clone()
                } else {
                    let view = Arc::new(Mutex::new(DirView::from(parent.clone())?));
                    history.insert(parent, view.clone());
                    view
                };

                view.lock().draw(stdscr(), LINES(), COLS());
            }
        } 
        else if c == b'l' as i32 {
            // Make we aren't trying to enter a file
            let child = view.lock().selected();
            if !child.filetype.is_dir() {
                continue;
            }
            let child = child.path.to_path_buf();

            wclear(stdscr());

            view = if history.get(&child).is_some() {
                history.get(&child).unwrap().clone()
            } else {
                    let view = Arc::new(Mutex::new(DirView::from(child.clone())?));
                    history.insert(child, view.clone());
                    view
            };

            view.lock().draw(stdscr(), LINES(), COLS());
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
