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

struct Entry {
    name: String,
    size: usize,
}

pub struct DirectoryView {
    dirs: Vec<Entry>,
    files: Vec<Entry>,
    focus: Rc<Cell<usize>>,
    last_offset: Cell<Vec2>,
}

impl Default for DirectoryView {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectoryView {
    fn new() -> Self {
        DirectoryView {
            dirs: Vec::new(),
            files: Vec::new(),
            focus: Rc::new(Cell::new(0)),
            last_offset: Cell::new(Vec2::zero()),
        }
    }

    fn from(path: &Path) -> Result<DirectoryView, Error> {
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
                view.dirs.push((name, size as usize));
            } else if meta.is_file() {
                let size = meta.len();
                view.files.push((name, size as usize));
            }
        }

        Ok(view)
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
                    Color::highlight(),
                    |printer| printer.print((0, i), dirs[i].name);
                );
            } else {
                printer.with_color(
                    Color::highlight(),
                    |printer| printer.print((0, i), dirs[i].name);
                );
            }
            printer.offset((0, i)).with_selection(
                i == self.focus(),
                |printer| {
                    if i != self.focus()
                        && !(self.enabled && printer.enabled)
                    {
                        printer.with_color(
                            ColorStyle::secondary(),
                            |printer| self.draw_item(printer, i),
                        );
                    } else {
                        self.draw_item(printer, i);
                    }
                },
            );
        }
    }

//     fn required_size(&mut self, _: Vec2) -> Vec2 {
//         // Items here are not compressible.
//         // So no matter what the horizontal requirements are,
//         // we'll still return our longest item.
//         let w = self
//             .items
//             .iter()
//             .map(|item| item.label.width())
//             .max()
//             .unwrap_or(1);
//         if self.popup {
//             Vec2::new(w + 2, 1)
//         } else {
//             let h = self.items.len();

//             Vec2::new(w, h)
//         }
//     }

//     fn on_event(&mut self, event: Event) -> EventResult {
//         if self.popup {
//             self.on_event_popup(event)
//         } else {
//             self.on_event_regular(event)
//         }
//     }

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

