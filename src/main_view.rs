use crate::DirectoryView;
use cursive::{
    event::{Event, EventResult},
    vec::Vec2,
    view::View,
    Printer,
};
use failure::Error;
use parking_lot::RwLock;
use std::{path::PathBuf, sync::Arc};
use crate::view_ratio::ViewRatio;

pub(crate) struct MainView {
    views: Vec<Arc<RwLock<DirectoryView>>>,
    ratios: ViewRatio,
}

impl MainView {
    fn new() -> Self {
        MainView { 
            views: Vec::new(),
            ratios: ViewRatio::default(),
        }
    }

    pub(crate) fn enter_dir(&mut self) {
        if let Some(last) = &self.views.last() {
            let focus = last.read().focus;
            // This is temporary, only allows you to enter directories
            // Cannot open files, yet.
            if focus >= last.read().dirs.len() {
                return;
            }

            let path = last.read().dirs[focus].path.clone();
            match DirectoryView::try_from(path, true, None) {
                Ok(view) => {
                    last.write();

                    view.write().get_sizes();
                    self.views.push(view);
                }
                Err(_) => {}
            }
        }
    }

    pub(crate) fn leave_dir(&mut self) {
        if self.views.len() < 2 {
            return;
        }

        self.views.pop();

        if let Some(last) = self.views.last() {
            last.write().enabled = true;
            if !last.read().has_sizes {
                last.write().get_sizes();
            }
        }
    }

    fn build_views_history(&mut self, path: PathBuf, _child: Option<PathBuf>) {
        match path.parent() {
            Some(parent_path) => match DirectoryView::try_from(parent_path.to_path_buf(), false, Some(path.clone())) {
                Ok(parent) => {
                    self.views.insert(0, parent);
                    self.build_views_history(parent_path.to_path_buf(), Some(path));
                }
                Err(_) => {}
            },
            None => return,
        }
    }

    pub(crate) fn try_from(path: PathBuf) -> Result<Self, Error> {
        let mut main_view = MainView::new();
        let main = DirectoryView::try_from(path.clone(), true, None)?;

        main_view.views.push(main);
        main_view.build_views_history(path, None);

        Ok(main_view)
    }
}


impl View for MainView {
    fn draw(&self, printer: &Printer) {
        let width = printer.size.x / self.ratios.sum() as usize;
        match self.views.len() {
            0 => return,
            1 => {
                let main_printer = printer.offset((width, 0)).cropped((width * (self.ratios.main as usize), printer.size.y));
                self.views[0].read().draw(&main_printer);
            },
            _ => {
                let parent_printer = printer.offset((0, 0)).cropped((width * (self.ratios.parent as usize), printer.size.y));
                let main_printer = printer.offset((width, 0)).cropped((width * (self.ratios.main as usize), printer.size.y));

                self.views[self.views.len() - 1].read().draw(&main_printer);
                self.views[self.views.len() - 2]
                    .read()
                    .draw(&parent_printer);
            }
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        match self.views.len() {
            0 => Vec2::zero(),
            1 => self.views[0].write().required_size(Vec2::zero()),
            _ => {
                let main = self.views[self.views.len() - 1]
                    .write()
                    .required_size(Vec2::zero());
                let parent = self.views[self.views.len() - 2]
                    .write()
                    .required_size(Vec2::zero());
                main + parent
            }
        }
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
                _ => {
                    if let Some(last) = &self.views.last() {
                        last.write().on_event(event)
                    } else {
                        EventResult::Ignored
                    }
                }
            },
            _ => {
                if let Some(last) = &self.views.last() {
                    last.write().on_event(event)
                } else {
                    EventResult::Ignored
                }
            }
        }
    }
}
