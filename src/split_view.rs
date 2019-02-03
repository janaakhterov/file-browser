use crate::{color_pair::ColorPair, entry::Entry, SETTINGS, VIEW_CACHE};
use cursive::{
    event::{Event, EventResult, Key},
    theme::{BaseColor, Color, ColorStyle, ColorType},
    vec::Vec2,
    view::View,
    Printer,
};
use failure::Error;
use futures01::{
    future::{poll_fn, Future},
    stream::Stream,
};
use parking_lot::Mutex;
use std::{cell::Cell, path::PathBuf, sync::Arc};
use tokio::runtime::Runtime;
use tokio_fs::read_dir;

pub struct SplitView {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub selected: usize,
    pub last_offset: Mutex<Cell<usize>>,
}

impl SplitView {
    pub fn try_from(path: PathBuf) -> Result<Arc<Mutex<Self>>, Error> {
        if let Some(cached) = VIEW_CACHE.lock().get(&path) {
            return Ok(cached.clone());
        }

        let entries = read_dir(path.clone())
            .into_stream()
            .flatten()
            .filter(|entry| {
                entry.file_name().into_string().is_ok()
                    && poll_fn(move || entry.poll_metadata()).wait().is_ok()
            })
            .map(|entry| {
                let path = entry.path();
                let filename = entry.file_name().into_string().unwrap();
                let metadata = poll_fn(move || entry.poll_metadata()).wait().unwrap();
                let filetype = metadata.file_type();
                let mime = mime_guess::guess_mime_type(&path);
                let permissions = metadata.permissions();
                let color = ColorPair::new(&filetype, &permissions);

                let first_char = match filename.chars().next() {
                    Some(v) => v,
                    None => return None,
                };

                if !SETTINGS.show_hidden && first_char == '.' {
                    return None;
                }

                Some(Entry {
                    path,
                    filename,
                    metadata,
                    filetype,
                    permissions,
                    mime,
                    color,
                })
            })
            .filter(|entry| entry.is_some())
            .map(|entry| entry.unwrap())
            .collect();

        let mut entries = Runtime::new()?.block_on(entries)?;
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

        let split_view = Arc::new(Mutex::new(SplitView {
            path: path.clone(),
            entries,
            selected,
            last_offset: Mutex::new(Cell::new(0)),
        }));

        VIEW_CACHE.lock().insert(path, split_view.clone());

        Ok(split_view.clone())
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

    pub fn change_selected_to(&mut self, path: PathBuf) {
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.path == path {
                self.selected = i;
                break;
            }
        }
    }

    pub fn selected(&self) -> Option<Entry> {
        if self.entries.len() > 0 {
            Some(self.entries[self.selected].clone())
        } else {
            None
        }
    }
}

impl View for SplitView {
    fn draw(&self, printer: &Printer) {
        if self.entries.len() == 0 {
            printer.with_color(
                ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                    ColorType::Color(Color::Dark(BaseColor::Red)),
                ),
                |printer| {
                    printer.print((0, 0), "empty");
                },
            );
            return;
        }

        let start = if self.last_offset.lock().get() > self.selected {
            self.selected
        } else if self.last_offset.lock().get() + printer.size.y - 1 < self.selected {
            self.selected - printer.size.y + 1
        } else {
            self.last_offset.lock().get()
        };

        self.last_offset.lock().set(start);

        let mut j = 0;
        for i in 0..printer.size.y {
            let cur = start.saturating_add(i);
            if cur < self.entries.len() {
                let element = &self.entries[cur];
                if !SETTINGS.show_hidden
                    && element.filename.chars().next().is_some()
                    && element.filename.chars().next().unwrap() == '.'
                {
                    continue;
                }

                let color = if cur == self.selected {
                    element.color.highlight
                } else {
                    element.color.default
                };

                printer.with_color(color, |printer| {
                    printer.print((0, j), &element.filename);
                    if element.filename.len() < printer.size.x - 1 {
                        printer.print_hline(
                            (element.filename.len(), j),
                            printer.size.x - element.filename.len(),
                            &" ",
                        );
                    }
                });
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

    fn required_size(&mut self, _constrait: Vec2) -> Vec2 {
        let w = self
            .entries
            .iter()
            .map(|entry| entry.filename.len())
            .max()
            .unwrap_or(1);
        Vec2::new(w, self.entries.len())
    }
}
