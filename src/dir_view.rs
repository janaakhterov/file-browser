use crate::{entry::Entry, SETTINGS};
use failure::Error;
use std::{cell::Cell, path::PathBuf, rc::Rc};

use futures01::{
    future::{poll_fn, Future},
    stream::Stream,
};

use tokio::runtime::Runtime;
use tokio_fs::read_dir;
use ncurses::*;
use crate::colors::*;

pub struct DirView {
    // Path to this directory that we're viewing
    pub path: PathBuf,

    // Each child file and directory
    pub entries: Vec<Entry>,

    // The currently focused element
    pub selected: usize,

    // The last offset used for printing. This determines if we
    // need to scroll up/down based on last position.
    pub last_offset: usize,
}

impl DirView {
    pub fn from(path: PathBuf) -> Result<Self, Error> {
        let mut rt = Runtime::new()?;
        let entries = read_dir(path.clone())
            .into_stream()
            .flatten()
            .filter(|entry| {
                if !entry.file_name().into_string().is_ok() {
                    return false;
                }
                let metadata = poll_fn(move || entry.poll_metadata()).wait();

                metadata.is_ok() && metadata.unwrap().modified().is_ok()
            })
            .map(|entry| {
                let path = entry.path();
                let filename = entry.file_name().into_string().unwrap();
                let metadata = poll_fn(move || entry.poll_metadata()).wait().unwrap();
                let permissions = metadata.permissions();
                let modified = metadata.modified().unwrap();
                let filetype = metadata.file_type();

                let color_pair = if filetype.is_dir() {
                    (DIR_COLOR, DIR_SELECTED_COLOR)
                } else if filetype.is_file() {
                    (FILE_COLOR, FILE_SELECTED_COLOR)
                } else if filetype.is_symlink() {
                    (LINK_COLOR, LINK_SELECTED_COLOR)
                } else {
                    (FILE_COLOR, FILE_SELECTED_COLOR)
                };

                let first_char = match filename.chars().next() {
                    Some(v) => v,
                    None => return None,
                };

                if !SETTINGS.show_hidden && first_char == '.' {
                    return None;
                }

                Some(Entry {
                    path,
                    metadata,
                    modified,
                    permissions,
                    filetype,
                    filename,
                    color_regular: color_pair.0,
                    color_selected: color_pair.1,
                })
            })
            .filter(|entry| entry.is_some())
            .map(|entry| entry.unwrap())
            .collect();

        let mut entries = rt.block_on(entries)?;
        entries.sort();

        Ok(DirView {
            path,
            entries,
            selected: 0,
            last_offset: 0,
        })
    }

    pub fn change_selected_by(&mut self, difference: i64) {
        let focus = if difference > 0 {
            if self.selected.saturating_add(difference as usize) >= self.entries.len() {
                self.entries.len().saturating_sub(1)
            } else {
                self.selected.saturating_add(difference as usize)
            }
        } else if difference < 0 {
            self.selected.saturating_sub(difference.abs() as usize)
        } else {
            self.selected
        };
        self.selected = focus;
    }

    pub fn draw(&mut self, win: WINDOW, lines: i32, cols: i32) {
        let lines = lines as usize;

        let start = if self.last_offset > self.selected {
            self.selected
        } else if self.last_offset.saturating_add(lines.saturating_sub(1)) < self.selected {
            self.selected.saturating_sub(lines.saturating_add(1))
        } else {
            self.last_offset
        };

        self.last_offset = start;

        wclear(win);

        for i in 0..lines {
            let cur = start.saturating_add(i);

            if cur >= self.entries.len() {
                break;
            }

            let element = &self.entries[cur];
            if !SETTINGS.show_hidden
                && element.filename.chars().next().is_some()
                && element.filename.chars().next().unwrap() == '.'
            {
                continue;
            }

            let color = if cur == self.selected {
                element.color_selected
            } else {
                element.color_regular
            };

            attron(COLOR_PAIR(color));
            mvwaddnstr(win, i as i32, 0, &element.filename, cols);

            if element.filename.len() < cols as usize {
                hline(' ' as u64, cols - element.filename.len() as i32);
            }

            attroff(COLOR_PAIR(color));
        }

        wrefresh(win);
    }
}
