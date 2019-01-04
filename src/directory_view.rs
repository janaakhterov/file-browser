use cursive::align::{Align};
use cursive::event::{Event, EventResult, Key};
use cursive::vec::Vec2;
use cursive::view::{View};
use cursive::Printer;
use failure::Error;
use std::cell::Cell;
use std::cmp;
use std::fs::read_dir;
use std::path::Path;
use std::rc::Rc;
use std::result::Result;
#[macro_use]
use crate::print_full_width;
use crate::palette::Palette;
use std::os::unix::fs::PermissionsExt;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Entry {
    name: String,
    permissions: u32,
    ext: Option<String>,
    size: usize,
}

pub struct DirectoryView {
    dirs: Vec<Entry>,
    files: Vec<Entry>,
    focus: Rc<Cell<usize>>,
    palette: Palette,
    align: Align,
    last_offset: Cell<Vec2>,
}

impl DirectoryView {
    fn new() -> Self {
        DirectoryView {
            dirs: Vec::new(),
            files: Vec::new(),
            focus: Rc::new(Cell::new(0)),
            palette: Palette::new(),
            align: Align::top_left(),
            last_offset: Cell::new(Vec2::zero()),
        }
    }

    pub fn from(path: &Path) -> Result<DirectoryView, Error> {
        let mut view = DirectoryView::new();

        for entry in read_dir(path)?
            .into_iter()
            .filter(Result::is_ok)
            .map(Result::unwrap)
        {
            let meta = entry.metadata()?;

            let name = match entry.file_name().into_string() {
                Ok(v) => v,
                // Err(err) => return err_msg("Failed to read file name"),
                Err(_) => "failed to load filename".to_string(),
            };

            let permissions = meta.permissions().mode();

            let ext = match entry.path().extension() {
                Some(s) => {
                    match s.to_str() {
                        Some(s) => Some(s.to_string()),
                        None => None,
                    }
                },
                None => None,
            };

            let size = if meta.is_dir() {
                read_dir(&Path::new(&entry.path()))?
                    .into_iter()
                    .filter(Result::is_ok)
                    .map(Result::unwrap)
                    .collect::<Vec<_>>()
                    .len() as usize
            } else if meta.is_file() {
                meta.len() as usize
            } else {
                0 as usize
            };

            match meta.is_dir() {
                true => &mut view.dirs,
                false => &mut view.files,
            }.push(Entry {
                name,
                permissions,
                ext,
                size,
            });
        }

        view.dirs.sort();
        view.files.sort();

        Ok(view)
    }

    pub fn focus_first(&mut self) {
        self.focus.set(0);
    }

    pub fn focus_last(&mut self) {
        self.focus.set(self.total_list_size() - 1)
    }

    pub fn focus(&self) -> usize {
        self.focus.get()
    }

    pub fn total_list_size(&self) -> usize {
        self.dirs.len() + self.files.len()
    }

    pub fn change_focus_by(&mut self, difference: i64) {
        let focus = self.focus.get();
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
        self.focus.set(new_focus);
    }
}

impl View for DirectoryView {
    fn draw(&self, printer: &Printer) {
        self.last_offset.set(printer.offset);
        let h = self.dirs.len() + self.files.len();
        let offset = self.align.v.get_offset(h, printer.size.y);
        let printer = &printer.offset((0, offset));
        let dirs_len = self.dirs.len();

        for i in 0..dirs_len {
            let name = &self.dirs[i].name;

            if i == self.focus() {
                printer.with_color(self.palette.dir_high, print_full_width!(name, i));
            } else {
                printer.with_color(self.palette.dir, print_full_width!(name, i));
            }
        }

        for i in 0..self.files.len() {
            let name = &self.files[i].name;
            let pos = i + dirs_len;

            if pos == self.focus() {
                printer.with_color(self.palette.file_high, print_full_width!(name, pos));
            } else {
                printer.with_color(self.palette.file, print_full_width!(name, pos));
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
            Event::Key(Key::Up) => self.change_focus_by(1),
            Event::Key(Key::Down) => self.change_focus_by(-1),
            Event::Key(Key::PageUp) => self.change_focus_by(10),
            Event::Key(Key::PageDown) => self.change_focus_by(-10),
            Event::Key(Key::Home) => self.focus.set(0),
            Event::Key(Key::End) => self.focus.set(self.total_list_size() - 1),
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
