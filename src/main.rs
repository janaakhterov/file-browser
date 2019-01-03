use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use failure::Error;
use cursive::Cursive;
use cursive::views::SelectView;
use cursive::views::ScrollView;
use cursive::views::IdView;
use cursive::views::BoxView;
use cursive::event::Event;
use cursive::event::Key;
use cursive::theme::ColorStyle;
use cursive::utils::span::SpannedString;
use cursive::theme::Style;
use crate::directory_view::DirectoryView;

mod pallete;
mod directory_view;
#[macro_use]
mod macros;

fn main() -> Result<(), Error> {
    let mut siv = Cursive::ncurses();

    siv.load_theme_file("styles.toml").unwrap();

    let mut dirs_view = DirectoryView::from(Path::new("/home/daniel"))?;

    siv.add_fullscreen_layer(dirs_view);
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
    Ok(())
}
