use cursive::align::{Align, HAlign, VAlign};
use cursive::direction::Direction;
use cursive::event::{Callback, Event, EventResult, Key, MouseButton, MouseEvent};
use cursive::menu::MenuTree;
use cursive::rect::Rect;
use std::borrow::Borrow;
use std::cell::Cell;
use std::cmp::min;
use std::rc::Rc;
use cursive::theme::ColorStyle;
use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::utils::markup::StyledString;
use cursive::vec::Vec2;
use cursive::view::{Position, View};
use cursive::views::MenuPopup;
use cursive::Cursive;
use cursive::Printer;
use cursive::With;
use failure::Error;
use failure::err_msg;
use std::fs::read_dir;
use std::path::Path;
use std::result::Result;
use std::cmp;
#[macro_use]
use crate::print_full_width;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Entry {
    name: String,
    size: usize,
}

pub struct DirectoryView {
    dirs: Vec<Entry>,
    files: Vec<Entry>,
    focus: Rc<Cell<usize>>,
    dir_color: ColorStyle,
    dir_highlight_color: ColorStyle,
    file_color: ColorStyle,
    file_highlight_color: ColorStyle,
    align: Align,
    last_offset: Cell<Vec2>,
}

impl DirectoryView {
    fn new() -> Self {
        let dir_color = ColorStyle::new(
            Color::Dark(BaseColor::Blue),
            Color::Dark(BaseColor::Black),
        );

        let dir_highlight_color = ColorStyle::new(
            dir_color.back,
            dir_color.front,
        );

        let file_color = ColorStyle::new(
            Color::Dark(BaseColor::White),
            Color::Dark(BaseColor::Black),
        );

        let file_highlight_color = ColorStyle::new(
            file_color.back,
            file_color.front,
        );

        DirectoryView {
            dirs: Vec::new(),
            files: Vec::new(),
            focus: Rc::new(Cell::new(0)),
            dir_color,
            dir_highlight_color,
            file_color,
            file_highlight_color,
            align: Align::top_left(),
            last_offset: Cell::new(Vec2::zero()),
        }
    }

    pub fn from(path: &Path) -> Result<DirectoryView, Error> {
        let mut view = DirectoryView::new();

        for entry in read_dir(path)?
            .into_iter()
                .filter(Result::is_ok)
                .map(Result::unwrap) {
            let meta = entry.metadata()?;
            let name = match entry.file_name().into_string() {
                Ok(v) => v,
                // Err(err) => return err_msg("Failed to read file name"),
                Err(_) => "failed to load filename".to_string(),
            };

            if meta.is_dir() {
                let size = read_dir(&Path::new(&entry.path()))?
                                         .into_iter()
                                         .filter(Result::is_ok)
                                         .map(Result::unwrap)
                                         .collect::<Vec<_>>()
                                         .len();
                view.dirs.push(Entry {
                    name, 
                    size: size as usize,
                });
            } else if meta.is_file() {
                let size = meta.len();
                view.files.push(Entry {
                    name, 
                    size: size as usize,
                });
            }
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
                printer.with_color(
                    self.dir_highlight_color,
                    print_full_width!(name, i),
                );
            } else {
                printer.with_color(
                    self.dir_color,
                    print_full_width!(name, i),
                );
            }
        }

        for i in 0..self.files.len() {
            let name = &self.files[i].name;
            let pos = i + dirs_len;

            if pos == self.focus() {
                printer.with_color(
                    self.file_highlight_color,
                    print_full_width!(name, pos),
                );
            } else {
                printer.with_color(
                    self.file_color,
                    print_full_width!(name, pos),
                );
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        let h = self.dirs.len() + self.files.len();

        let w = { 
            cmp::max(self.dirs
                    .iter()
                    .map(|dir| dir.name.len())
                    .max()
                    .unwrap_or(1),
                self.files
                    .iter()
                    .map(|file| file.name.len())
                    .max()
                    .unwrap_or(1))
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
            Event::Char(c) => {
                match c {
                    'j' => self.change_focus_by(1),
                    'k' => self.change_focus_by(-1),
                    _ => return EventResult::Ignored,
                }
            },
            _ => return EventResult::Ignored,
        }

        EventResult::Consumed(None)
    }
}

