#[macro_use]
extern crate once_cell;

use crate::{settings::Settings, split_view::SplitView};
use config::Config;
use cursive::{views::BoxView, Cursive};
use failure::Error;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::{collections::HashMap, env::current_dir, path::PathBuf, result::Result, sync::Arc};
use tab_view::TabView;

pub mod entry;
pub mod settings;
pub mod split_view;
pub mod tab_view;

static SETTINGS: Lazy<Settings> = sync_lazy! {
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

static VIEW_CACHE: Lazy<Mutex<HashMap<PathBuf, Arc<Mutex<SplitView>>>>> = sync_lazy! {
    Mutex::new(HashMap::new())
};

fn main() -> Result<(), Error> {
    let mut siv = Cursive::ncurses();
    siv.load_theme_file("styles.toml").unwrap();

    let view = BoxView::with_full_screen(TabView::try_from(current_dir()?)?);

    siv.add_fullscreen_layer(view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}
