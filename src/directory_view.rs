use cursive::{
    align::Align,
    event::{Event, EventResult, Key},
    vec::Vec2,
    view::View,
    Printer,
};
use failure::Error;
use std::{cmp, result::Result};
#[macro_use]
use crate::print_full_width_with_selection;
use crate::{color_pair::ColorPair, entry::Entry, print_full_width};
use core::convert::TryFrom;
use number_prefix::{binary_prefix, Prefixed, Standalone};
use parking_lot::RwLock;
use std::{cmp::Ordering, path::PathBuf};
use std::thread;
use tokio_fs::read_dir;
use tokio_fs::metadata;
use futures::future::Future;
use std::sync::Arc;
use futures::stream::Stream;
use tokio_fs::read_link;
use std::fs::Metadata;

pub(crate) struct DirectoryView {
    pub(crate) path: PathBuf,
    pub(crate) dirs: Vec<Entry>,
    pub(crate) files: Vec<Entry>,
    focus: usize,
    align: Align,
    last_offset: RwLock<usize>,
}

pub(crate) fn search_vec(v: &Vec<Entry>, entry: &Entry) -> usize {
    let mut l: usize = 0;
    let mut m: usize = 0;
    let mut r: usize = v.len().checked_sub(1).unwrap_or_else(|| 0);

    if v.len() < 1 {
        return 0;
    }

    match v[r].cmp(entry) {
        Ordering::Less => return r,
        _ => {}
    }

    match v[0].cmp(entry) {
        Ordering::Greater => return 0,
        _ => {}
    }

    while l <= r {
        m = (((l + r) / 2) as f64).floor() as usize;
        if v[m] < *entry {
            let temp = m.checked_add(1);
            if temp.is_none() {
                return 0;
            }
            l = temp.unwrap();
        } else if v[m] > *entry {
            let temp = m.checked_sub(1);
            if temp.is_none() {
                return 0;
            }
            r = temp.unwrap();
        } else {
            // Shouldn't be possible to get here unless two entries have the same name
            // which shouldn't happen since names are file names and duplicate files
            // are not possible;
            return m as usize;
        }
    }
    // Usually would end up here
    return m as usize;
}

pub(crate) fn insert(v: &mut Vec<Entry>, entry: Entry) {
    let index = search_vec(&v, &entry);
    v.insert(index, entry);
}

impl DirectoryView {
    fn new(path: PathBuf) -> Self {
        DirectoryView {
            path,
            dirs: Vec::new(),
            files: Vec::new(),
            focus: 0,
            align: Align::top_left(),
            last_offset: RwLock::new(0),
        }
    }

    pub(crate) fn focus_path(&mut self, path: PathBuf) {
        for (i, entry) in self.dirs.iter().enumerate() {
            if entry.path == path {
                self.focus = i;
            }
        }

        for (i, entry) in self.files.iter().enumerate() {
            if entry.path == path {
                self.focus = i + self.dirs.len();
            }
        }
    }

    fn size(entry: PathBuf, meta: &Metadata) -> String {
        let filetype = meta.file_type();

        if filetype.is_dir() {
            let count = Arc::new(RwLock::new(0 as usize));
            let c = count.clone();
            let fut = read_dir(entry)
                .flatten_stream()
                .for_each(move |_| {
                    let cur = *c.read();
                    *c.write() = cur + 1;
                    Ok(())
                })
                .map_err(|_| {});

            tokio::run(fut);

            match Arc::try_unwrap(count) {
                Ok(count) => count.into_inner().to_string(),
                Err(_) => "".to_string(),
            }
        } else if filetype.is_file() {
            match binary_prefix(meta.len() as f64) {
                Standalone(bytes) => format!("{} B", bytes),
                Prefixed(prefix, n) => format!("{:.0} {}B", n, prefix),
            }
        } else if filetype.is_symlink() {
            match read_link(entry).wait() {
                Ok(link) => {
                    let meta = match metadata(link.clone()).wait() {
                        Ok(meta) => meta,
                        Err(_) => return "Broken Link".to_string(),
                    };
                    DirectoryView::size(link, &meta)
                },
                Err(_) => "Broken Link".to_string(),
            }
        } else {
            "Error".to_string()
        }
    }

    pub(crate) fn focus(&self) -> usize {
        self.focus
    }

    pub(crate) fn change_focus_by(&mut self, difference: i64) {
        let focus = self.focus;
        let new_focus = if difference > 0 {
            if focus + difference as usize >= self.total_list_size() {
                (self.total_list_size() - 1) as usize
            } else {
                focus.saturating_add(difference as usize)
            }
        } else if difference < 0 {
            focus.saturating_sub(difference.abs() as usize)
        } else {
            focus
        };
        self.focus = new_focus;
    }

    pub(crate) fn total_list_size(&self) -> usize {
        self.dirs.len() + self.files.len()
    }

    pub(crate) fn try_from(path: PathBuf) -> Result<Arc<RwLock<Self>>, Error> {
        let view = Arc::new(RwLock::new(DirectoryView::new(path.clone())));
        let v = view.clone();
        thread::spawn(move || {
            let fut = read_dir(path.clone())
                .flatten_stream()
                .for_each(move |entry| {
                    let path = entry.path();
                    let size = Arc::new(RwLock::new(String::new()));

                    let meta = match metadata(entry.path()).wait() {
                        Ok(meta) => meta,
                        Err(_) => return Ok(()),
                    };

                    let filetype = meta.file_type();

                    let s = size.clone();
                    let m = meta.clone();
                    let p = entry.path().clone();
                    thread::spawn(move || {
                        let filetype = m.file_type();
                        let size = DirectoryView::size(p, &m);
                        let size = if filetype.is_symlink() {
                            format!("-> {}", size)
                        } else {
                            size
                        };
                        *s.write() = size;

                    });

                    let name = match entry.file_name().into_string() {
                        Ok(name) => name,
                        Err(_) => return Ok(()),
                    };

                    let color = ColorPair::new(&entry, &meta).unwrap_or_else(|_| ColorPair::default());

                    let entry = Entry {
                        path,
                        name,
                        size,
                        color,
                    };

                    if meta.is_dir() {
                        insert(&mut v.write().dirs, entry);
                    } else {
                        insert(&mut v.write().files, entry);
                    }

                    Ok(())
                })
                .map_err(|_| {});

            tokio::run(fut);
        });

        Ok(view)
    }
}

impl View for DirectoryView {
    fn draw(&self, printer: &Printer) {
        let height = self.dirs.len() + self.files.len();
        let offset = self.align.v.get_offset(height, printer.size.y);
        let printer = &printer.offset((0, offset));
        let focus = self.focus();

        // Which element should we start at to make sure the focused element
        // is in view.
        let start = if *self.last_offset.read() > focus {
            focus
        } else if *self.last_offset.read() + printer.size.y - 1 < focus {
            focus - printer.size.y + 1
        } else {
            *self.last_offset.read()
        };

        // Set the current start as the next offset
        *self.last_offset.write() = start;

        // Loop through all the lines in the printer
        // Either print a file at the current line or a directory
        // Based on the current focus
        for i in 0..printer.size.y {
            let element = i + start;
            if element < self.dirs.len() {
                let name = &self.dirs[element].name;
                let color = &self.dirs[element].color;
                let size = &self.dirs[element].size;
                print_full_width_with_selection!(printer, element, focus, name, size, color, i);
            } else if element - self.dirs.len() < self.files.len() {
                let name = &self.files[element - self.dirs.len()].name;
                let color = &self.files[element - self.dirs.len()].color;
                let size = &self.files[element - self.dirs.len()].size;
                print_full_width_with_selection!(printer, element, focus, name, size, color, i);
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let h = self.dirs.len() + self.files.len();

        let w = {
            cmp::max(
                self.dirs
                    .iter()
                    .map(|dir| dir.name.len())
                    .max()
                    .unwrap_or(1),
                self.files
                    .iter()
                    .map(|file| file.name.len())
                    .max()
                    .unwrap_or(1),
            )
        };

        Vec2::new(w, h)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Up) => self.change_focus_by(-1),
            Event::Key(Key::Down) => self.change_focus_by(1),
            Event::Key(Key::PageUp) => self.change_focus_by(-10),
            Event::Key(Key::PageDown) => self.change_focus_by(10),
            Event::Key(Key::Home) => self.focus = 0,
            Event::Key(Key::End) => self.focus = self.total_list_size() - 1,
            Event::Char(c) => match c {
                'j' => self.change_focus_by(1),
                'k' => self.change_focus_by(-1),
                _ => return EventResult::Ignored,
            },
            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(None)
    }
}
