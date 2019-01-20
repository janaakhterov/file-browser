use failure::Error;
use std::path::PathBuf;
use crate::entry::Entry;
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::Printer;
use std::fs::read_dir;
use std::rc::Rc;
use std::cell::Cell;

pub struct DirView {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub last_offset: Rc<Cell<usize>>,
}

impl DirView {
    pub fn from(path: PathBuf) -> Result<Self, Error> {
        let mut entries: Vec<Entry> = read_dir(&path)?
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .filter(|entry| {
                entry.metadata().is_ok() &&
                entry.file_type().is_ok() &&
                entry.file_name().into_string().is_ok()
            })
            .map(|entry| {
                let path = entry.path();
                let metadata = entry.metadata().unwrap();
                let filetype = entry.file_type().unwrap();
                let filename = entry.file_name().into_string().unwrap();

                Entry {
                    path,
                    metadata,
                    filetype,
                    filename,
                }
            })
            .collect();
        entries.sort();

        Ok(DirView {
            path,
            entries,
            selected: 0,
            last_offset: Rc::new(Cell::new(0)),
        })
    }
}

impl View for DirView {
    fn draw(&self, printer: &Printer) {
        let start = if self.last_offset.get() > self.selected {
            self.selected
        } else if self.last_offset.get() + printer.size.y - 1 < self.selected {
            self.selected - printer.size.y + 1
        } else {
            self.last_offset.get()
        };

        self.last_offset.set(start);

        for i in 0..printer.size.y {
            let element = i + start;
            if element < self.entries.len() {
                printer.print((0, i), &self.entries[element].filename);
            }
        }
    }

    fn required_size(&mut self, constrait: Vec2) -> Vec2 {
        let w = self.entries
            .iter()
            .map(|entry| entry.filename.len())
            .max()
            .unwrap_or(1);
        Vec2::new(w, self.entries.len())
    }
}
