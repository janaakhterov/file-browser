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

pub(crate) struct MainView {
    main: DirectoryView,
}

impl MainView {
    pub(crate) fn enter_dir(&mut self) {
        let focus = self.main.focus();
        if focus >= self.main.dirs.len() {
            return;
        }
        let path = self.main.dirs[focus].path.clone();
        let view = DirectoryView::try_from(path);
        if view.is_ok() {
            self.main = view.unwrap();
        }
    }

    pub(crate) fn leave_dir(&mut self) {
        let path = self.main.path.clone();
        let parent = path.parent();
        if parent.is_none() {
            return;
        }
        let parent = parent.unwrap();

        match DirectoryView::try_from(parent.to_path_buf()) {
            Ok(view) => {
                self.main = view;
                self.main.focus_path(path);
            }
            Err(_) => {}
        }
    }
}

impl TryFrom<PathBuf> for MainView {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let main = DirectoryView::try_from(path)?;

        Ok(MainView { main })
    }
}

impl View for MainView {
    fn draw(&self, printer: &Printer) {
        // let printer = &printer.inner_size((30, 10));
        self.main.draw(printer);
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.main.required_size(Vec2::zero())
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
                _ => self.main.on_event(event),
            },
            _ => self.main.on_event(event),
        }
    }
}
