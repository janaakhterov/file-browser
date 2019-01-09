use cursive::{
    align::Align,
    event::{Event, EventResult, Key},
    vec::Vec2,
    view::View,
    Printer,
};
use crate::DirectoryView;
use std::path::Path;
use failure::Error;
use std::convert::TryFrom;

pub(crate) struct MainView {
    main: DirectoryView,
}

impl TryFrom<&Path> for MainView {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
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
        self.main.on_event(event)
    }
}
