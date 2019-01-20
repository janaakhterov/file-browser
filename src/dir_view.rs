use failure::Error;
use std::path::PathBuf;
use crate::entry::Entry;
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::Printer;
use std::fs::read_dir;
use std::rc::Rc;
use std::cell::Cell;

use cursive::event::EventResult;
use cursive::event::Event;
use cursive::event::Key;
use cursive::theme::ColorStyle;

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
            let element = start.saturating_add(i);
            if element < self.entries.len() {
                if element == self.selected {
                    printer.with_color(
                        ColorStyle::highlight(),
                        |printer| printer.print((0, i), &self.entries[element].filename));
                } else {
                    printer.print((0, i), &self.entries[element].filename);
                }
            }
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Up) => self.change_selected_by(-1),
            Event::Key(Key::Down) => self.change_selected_by(1),
            Event::Key(Key::PageUp) => self.change_selected_by(-10),
            Event::Key(Key::PageDown) => self.change_selected_by(10),
            Event::Key(Key::Home) => self.selected = 0,
            Event::Key(Key::End) => self.selected = self.entries.len().saturating_sub(1),
            Event::Char(c) => match c {
                'j' => self.change_selected_by(1),
                'k' => self.change_selected_by(-1),
                _ => return EventResult::Ignored,
            },
            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(None)
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
