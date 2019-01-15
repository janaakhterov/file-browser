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
use crate::{color_pair::ColorPair, entry::Entry, print_empty, print_full_width};
use core::convert::TryFrom;
use cursive::theme::{BaseColor, Color, ColorStyle};
use futures::{future::Future, stream::Stream};
use number_prefix::{binary_prefix, Prefixed, Standalone};
use parking_lot::RwLock;
use std::{cmp::Ordering, fs::Metadata, path::PathBuf, sync::Arc, thread};
use tokio_fs::{metadata, read_dir, read_link};
use crate::size::Size;
use crate::size::SizeString;

pub(crate) struct DirectoryView {
    pub(crate) path: PathBuf,
    pub(crate) dirs: Vec<Entry>,
    pub(crate) files: Vec<Entry>,
    pub(crate) has_sizes: bool,
    focus: usize,
    enabled: bool,
    align: Align,
    last_offset: RwLock<usize>,
}

pub(crate) fn search_vec(v: &Vec<Entry>, entry: &Entry) -> usize {
    if v.len() < 1 {
        return 0;
    }

    let mut l: usize = 0;
    let mut m: usize = 0;
    let mut r: usize = v.len() - 1;

    match v[r].cmp(entry) {
        Ordering::Less => return r,
        _ => {}
    }

    match v[0].cmp(entry) {
        Ordering::Greater => return 0,
        _ => {}
    }

    while l <= r {
        m = (l + r) / 2;
        match v[m].cmp(entry) {
            Ordering::Greater => {
                let temp = m.checked_sub(1);
                if temp.is_none() {
                    return 0;
                }
                r = temp.unwrap();
            },
            Ordering::Less => {
                let temp = m.checked_add(1);
                if temp.is_none() {
                    return 0;
                }
                l = temp.unwrap();
            },
            Ordering::Equal => {
                // Shouldn't be possible to get here unless two entries have the same name
                // which shouldn't happen since names are file names and duplicate files
                // are not possible;
                return m as usize;
            },
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
    fn new(path: PathBuf, has_sizes: bool) -> Self {
        DirectoryView {
            path,
            dirs: Vec::new(),
            files: Vec::new(),
            has_sizes,
            focus: 0,
            enabled: true,
            align: Align::top_left(),
            last_offset: RwLock::new(0),
        }
    }

    pub(crate) fn disable(&mut self) {
        self.enabled = false;
    }

    pub(crate) fn enable(&mut self) {
        self.enabled = true;
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.enabled
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

    fn size(entry: PathBuf, meta: &Metadata) -> SizeString {
        let mut size = SizeString::new();
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

            size.size = Size::Usize(*count.read());
            size
        } else if filetype.is_file() {
            match binary_prefix(meta.len() as f64) {
                Standalone(bytes) => {
                    size.size = Size::Float(bytes);
                    size.suffix = "B".to_string();
                    size
                },
                Prefixed(suffix, bytes) => {
                    size.size = Size::Float(bytes);
                    size.suffix = format!("{}B", suffix.to_string());
                    size
                },
            }
        } else if filetype.is_symlink() {
            match read_link(entry.clone()).wait() {
                Ok(link) => {
                    if link == entry {
                        return size;
                    }

                    let new_meta = match metadata(link.clone()).wait() {
                        Ok(meta) => meta,
                        Err(_) => return size,
                    };
                    size = DirectoryView::size(link, &new_meta);
                    size.prefix = "->";
                    size
                },
                Err(_) => size,
            }
        } else {
            size
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

    pub(crate) fn get_sizes(&mut self) {
        for entry in self.dirs.iter() {
            let s = entry.size.clone();

            let path = entry.path.clone();
            let meta = match path.clone().metadata() {
                Ok(meta) => meta,
                Err(err) => return,
            };
            let filetype = meta.file_type();

            *s.write() = DirectoryView::size(path.clone(), &meta).to_string();
            self.has_sizes = true;
        }
    }

    pub(crate) fn try_from(path: PathBuf, show_size: bool) -> Result<Arc<RwLock<Self>>, Error> {
        let view = Arc::new(RwLock::new(DirectoryView::new(path.clone(), show_size)));
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

                    if show_size {
                        let m = meta.clone();
                        let p = path.clone();
                        let s = size.clone();
                        thread::spawn(move || {
                            *s.write() = DirectoryView::size(p, &m).to_string();
                        });
                    }

                    let name = match entry.file_name().into_string() {
                        Ok(name) => name,
                        Err(_) => return Ok(()),
                    };

                    let color =
                        ColorPair::new(&entry, &meta).unwrap_or_else(|_| ColorPair::default());

                    let entry = Entry {
                        path,
                        name,
                        size,
                        color,
                    };

                    if meta.is_dir() {
                        // insert(&mut v.write().dirs, entry);
                        // let len = v.read().dirs.len().checked_sub(1).unwrap_or_else(|| 0);
                        // v.write().dirs.insert(len, entry);
                        v.write().dirs.push(entry);
                        v.write().dirs.sort();
                    } else {
                        // insert(&mut v.write().files, entry);
                        // let len = v.read().files.len().checked_sub(1).unwrap_or_else(|| 0);
                        // v.write().files.insert(len, entry);
                        v.write().files.push(entry);
                        v.write().files.sort();
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
        let enabled = self.is_enabled();

        if height == 0 {
            let color = ColorStyle::new(Color::Dark(BaseColor::White), Color::Dark(BaseColor::Red));
            print_empty!(printer, color);
            return;
        }

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
                print_full_width_with_selection!(
                    printer, element, focus, enabled, name, size, color, i
                );
            } else if element - self.dirs.len() < self.files.len() {
                let name = &self.files[element - self.dirs.len()].name;
                let color = &self.files[element - self.dirs.len()].color;
                let size = &self.files[element - self.dirs.len()].size;
                print_full_width_with_selection!(
                    printer, element, focus, enabled, name, size, color, i
                );
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
