use crate::SETTINGS;
use failure::Error;
use std::path::PathBuf;
use crate::entry::Entry;
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::Printer;
use std::rc::Rc;
use std::cell::Cell;

use cursive::event::EventResult;
use cursive::event::Event;
use cursive::event::Key;
use cursive::theme::ColorStyle;

use futures01::future::Future;
use futures01::stream::Stream;
use futures01::future::poll_fn;

use tokio_fs::read_dir;
use tokio::runtime::Runtime;

pub struct DirView {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub last_offset: Rc<Cell<usize>>,
}

impl DirView {
    pub fn from(path: PathBuf) -> Result<Self, Error> {
        let mut rt = Runtime::new()?;
        let entries = read_dir(path.clone())
            .into_stream()
            .flatten()
            .filter(|entry| {
                entry.file_name().into_string().is_ok() &&
                poll_fn(move || entry.poll_metadata()).wait().is_ok()
            })
            .map(|entry| {
                let path = entry.path();
                let filename = entry.file_name().into_string().unwrap();
                let metadata = poll_fn(move || entry.poll_metadata()).wait().unwrap();
                let filetype = metadata.file_type();

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
                    filetype,
                    filename,
                })
            })
            .filter(|entry| { entry.is_some() })
            .map(|entry| { entry.unwrap() })
            .collect();
        let mut entries = rt.block_on(entries)?;
        entries.sort();

        let selected = if !SETTINGS.show_hidden {
            let mut selected = 0;
            for (i, entry) in entries.iter().enumerate() {
                let c = match entry.filename.chars().next() {
                    Some(v) => v,
                    None => continue,
                };
                if c == '.' {
                    continue;
                } else {
                    selected = i;
                    break;
                }
            }
            selected
        } else {
            0
        };

        Ok(DirView {
            path,
            entries,
            selected,
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

        let mut j = 0;
        for i in 0..printer.size.y {
            let cur = start.saturating_add(i);
            if cur < self.entries.len() {
                let element = &self.entries[cur];
                if !SETTINGS.show_hidden && 
                    element.filename.chars().next().is_some() && 
                    element.filename.chars().next().unwrap() == '.' {
                    continue;
                }

                if cur == self.selected {
                    printer.with_color(
                        ColorStyle::highlight(),
                        |printer| {
                            printer.print((0, j), &element.filename);
                            printer.print_hline((element.filename.len(), j), printer.size.x - element.filename.len(), &" ");
                        });
                } else {
                    printer.print((0, j), &element.filename);
                }
            }
            j = j + 1;
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
