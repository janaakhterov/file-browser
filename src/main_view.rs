use crate::DirectoryView;
use core::convert::TryFrom;
use cursive::{
    event::{Event, EventResult},
    vec::Vec2,
    view::View,
    Printer,
};
use failure::Error;
use std::path::PathBuf;
use parking_lot::RwLock;
use std::sync::Arc;

pub(crate) struct MainView {
    parent: Option<Arc<RwLock<DirectoryView>>>,
    main: Arc<RwLock<DirectoryView>>,
}

impl MainView {
    pub(crate) fn enter_dir(&mut self) {
        let focus = self.main.read().focus();
        if focus >= self.main.read().dirs.len() {
            return;
        }

        let path = self.main.read().dirs[focus].path.clone();
        let view = DirectoryView::try_from(path);
        if view.is_ok() {
            self.parent = Some(self.main.clone());
            self.main = view.unwrap();
        }
    }

    pub(crate) fn leave_dir(&mut self) {
        if let Some(parent) = &mut self.parent {
            self.main = parent.clone();
            self.main.write().enable();

            self.parent = match self.main.read().path.parent() {
                Some(path) => {
                    match DirectoryView::try_from(path.to_path_buf()) {
                        Ok(parent) => Some(parent),
                        Err(_) => None,
                    }
                }
                None => None,
            };
        }
    }
}

impl TryFrom<PathBuf> for MainView {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let main = DirectoryView::try_from(path.clone())?;
        let parent = match path.parent() {
            Some(path) => {
                match DirectoryView::try_from(path.to_path_buf()) {
                    Ok(parent) => {
                        parent.write().disable();
                        Some(parent)
                    },
                    Err(_) => None,
                }
            }
            None => None,
        };

        Ok(MainView { 
            parent,
            main,
        })
    }
}

impl View for MainView {
    fn draw(&self, printer: &Printer) {
        // let printer = &printer.inner_size((30, 10));

        if let Some(parent) = &self.parent {
            let width = printer.size.x/2 ;
            let parent_printer = printer
                .cropped((width, printer.size.y));
            let main_printer = printer
                .offset((width, 0))
                .cropped((width, printer.size.y));

            parent.read().draw(&parent_printer);
            self.main.read().draw(&main_printer);
        } else {
            self.main.read().draw(printer);
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.main.write().required_size(Vec2::zero())
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(c) => match c {
                'l' => {
                    self.enter_dir();
                    EventResult::Consumed(None)
                }
                'h' => {
                    self.leave_dir();
                    EventResult::Consumed(None)
                }
                _ => self.main.write().on_event(event),
            },
            _ => self.main.write().on_event(event),
        }
    }
}
