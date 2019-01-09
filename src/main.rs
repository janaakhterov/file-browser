#![feature(try_from)]
#[macro_use]
extern crate lazy_static;

use crate::directory_view::DirectoryView;
use config::Config;
use cursive::{views::BoxView, Cursive};
use failure::Error;
use parking_lot::Mutex;
use std::{path::PathBuf, result::Result};
use crate::main_view::MainView;
use std::convert::TryFrom;
use gag::Redirect;
use std::fs::File;

mod color_pair;
mod directory_view;
mod entry;
mod main_view;
#[macro_use]
mod macros;

lazy_static! {
    static ref SETTINGS: Mutex<Config> = {
        let mut config = Config::new();

        // TODO: Print error, but don't quit app
        match config.merge(config::File::with_name("settings.json")) {
            Ok(_) => {}
            Err(_) => {}
        }

        Mutex::new(config)
    };
}

fn main() -> Result<(), Error> {
    // let file = File::create("output.log")?;
    // let _ = Redirect::stderr(file)?;
    let mut siv = Cursive::ncurses();

    siv.load_theme_file("styles.toml").unwrap();

    let dirs_view =
        BoxView::with_full_screen(MainView::try_from(PathBuf::from("/"))?);

    siv.add_fullscreen_layer(dirs_view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}
