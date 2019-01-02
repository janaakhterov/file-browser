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
        DirectoryView {
            dirs: Vec::new(),
            files: Vec::new(),
            focus: Rc::new(Cell::new(0)),
            dir_color: ColorStyle::primary(),
            dir_highlight_color: ColorStyle::highlight(),
            file_color: ColorStyle::primary(),
            file_highlight_color: ColorStyle::highlight(),
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

    fn focus(&self) -> usize {
        self.focus.get()
    }
}


impl View for DirectoryView {
    fn draw(&self, printer: &Printer) {
        self.last_offset.set(printer.offset);
        let h = self.dirs.len() + self.files.len();
        let offset = self.align.v.get_offset(h, printer.size.y);
        let printer = &printer.offset((0, offset));

        for i in 0..self.dirs.len() {
            if i == self.focus() {
                printer.with_color(
                    self.dir_highlight_color,
                    |printer| { printer.print((0, i), &self.dirs[i].name); },
                );
            } else {
                printer.with_color(
                    self.dir_color,
                    |printer| { printer.print((0, i), &self.dirs[i].name); },
                );
            }
        }
        for i in 0..self.files.len() {
            if i + self.dirs.len()  == self.focus() {
                printer.with_color(
                    self.file_highlight_color,
                    |printer| { printer.print((0, i + self.dirs.len()), &self.files[i].name); },
                );
            } else {
                printer.with_color(
                    self.file_color,
                    |printer| { printer.print((0, i + self.dirs.len()), &self.files[i].name); },
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
        self.on_event_regular(event)
    }

//     fn take_focus(&mut self, _: Direction) -> bool {
//         self.enabled && !self.items.is_empty()
//     }

//     fn layout(&mut self, size: Vec2) {
//         self.last_size = size;
//     }

//     fn important_area(&self, size: Vec2) -> Rect {
//         self.selected_id()
//             .map(|i| Rect::from_size((0, i), (size.x, 1)))
//             .unwrap_or_else(|| Rect::from((0, 0)))
//     }
}

