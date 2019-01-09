use cursive::{
    align::Align,
    event::{Event, EventResult, Key},
    vec::Vec2,
    view::View,
    Printer,
};
use crate::DirectoryView;
use std::path::PathBuf;
use failure::Error;
use std::convert::TryFrom;

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
        let path = self.main.path.parent();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        let view = DirectoryView::try_from(path.to_path_buf());
        if view.is_ok() {
            let view = view.unwrap();
            self.main = view;
        }
    }
}

impl TryFrom<PathBuf> for MainView {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let main = DirectoryView::try_from(path)?;

        Ok(MainView {
            main
        })
    }
}

impl View for MainView {
    fn draw(&self, printer: &Printer) {
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
                },
                'h' => {
                    self.leave_dir();
                    EventResult::Consumed(None)
                }
                _ => self.main.on_event(event)
            },
            _ => self.main.on_event(event)
        }
    }
}
