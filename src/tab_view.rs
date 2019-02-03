use crate::split_view::SplitView;
use cursive::{
    event::{Event, EventResult, Key},
    vec::Vec2,
    view::View,
    Printer,
};
use failure::Error;
use parking_lot::Mutex;
use std::{path::PathBuf, sync::Arc};

pub struct TabView {
    parent: Option<Arc<Mutex<SplitView>>>,
    current: Arc<Mutex<SplitView>>,
    preview: Option<Arc<Mutex<SplitView>>>,
}

impl TabView {
    pub fn try_from(path: PathBuf) -> Result<Self, Error> {
        let parent = {
            if let Some(parent_path) = path.parent() {
                let parent = SplitView::try_from(parent_path.to_path_buf())?;
                parent.lock().change_selected_to(path.clone());
                Some(parent)
            } else {
                None
            }
        };

        let current = SplitView::try_from(path.clone())?;

        let preview = {
            if let Some(selected) = current.lock().selected() {
                Some(SplitView::try_from(selected.path)?)
            } else {
                None
            }
        };

        Ok(TabView {
            parent,
            current,
            preview,
        })
    }

    pub fn update_preview(&mut self) {
        if let Some(selected) = self.current.lock().selected() {
            if selected.filetype.is_dir() {
                self.preview = match SplitView::try_from(selected.path) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
            } else {
                self.preview = None;
            }
        }
    }

    pub fn enter_dir(&mut self) {
        if let Some(preview) = &self.preview {
            self.parent = Some(self.current.clone());
            self.current = preview.clone();
            self.update_preview();
        }
    }

    pub fn leave_dir(&mut self) {
        if let Some(parent) = &self.parent {
            self.current = parent.clone();
            let current_path = self.current.lock().path.clone();
            if let Some(path) = self.current.lock().path.parent() {
                self.parent = match SplitView::try_from(path.to_path_buf()) {
                    Ok(parent) => {
                        parent.lock().change_selected_to(current_path);
                        Some(parent)
                    }
                    Err(_) => None,
                }
            } else {
                self.parent = None;
            }
            self.update_preview();
        }
    }
}

impl View for TabView {
    fn draw(&self, printer: &Printer) {
        let parent_printer = printer.cropped((printer.size.x / 4, printer.size.y));
        let current_printer = printer
            .offset((printer.size.x / 4, 0))
            .cropped((printer.size.x / 2, printer.size.y));
        let preview_printer = printer
            .offset((3 * printer.size.x / 4, 0))
            .cropped((printer.size.x / 4, printer.size.y));

        if let Some(parent) = &self.parent {
            parent.lock().draw(&parent_printer);
        }

        self.current.lock().draw(&current_printer);

        if let Some(preview) = &self.preview {
            preview.lock().draw(&preview_printer);
        }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let parent = if self.parent.is_some() {
            self.parent
                .clone()
                .unwrap()
                .lock()
                .required_size(Vec2::zero())
        } else {
            Vec2::zero()
        };

        let current = self.current.lock().required_size(Vec2::zero());

        let preview = if self.preview.is_some() {
            self.preview
                .clone()
                .unwrap()
                .lock()
                .required_size(Vec2::zero())
        } else {
            Vec2::zero()
        };

        parent + current + preview
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Left) => self.leave_dir(),
            Event::Char(c) => match c {
                'h' => self.leave_dir(),
                'l' => self.enter_dir(),
                _ => {
                    let event = self.current.lock().on_event(event);
                    match event {
                        EventResult::Consumed(_) => self.update_preview(),
                        EventResult::Ignored => return EventResult::Ignored,
                    }
                }
            },
            _ => return self.current.lock().on_event(event),
        }

        EventResult::Consumed(None)
    }
}
