use crate::{split_view::SplitView, VIEW_CACHE};
use cursive::{vec::Vec2, view::View, views::BoxView, Printer};
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
        let mut parent = None;
        if let Some(parent_path) = path.parent() {
            parent = Some(SplitView::try_from(parent_path.to_path_buf())?);
        }

        let current = SplitView::try_from(path.clone())?;
        let preview = None;

        Ok(TabView {
            parent,
            current,
            preview,
        })
    }
}

impl View for TabView {
    fn draw(&self, printer: &Printer) {
        // if let Some(parent) = self.parent {
        //     parent.draw();
        // }

        // self.current.draw();

        // if let Some(preview) = self.preview {
        //     preview.draw();
        // }
    }

    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        // let parent = if self.parent.is_some() {
        //     let parent = self.parent.unwrap();
        // // *parent.clone().lock().requied_size();
        // } else {
        //     (0, 0)
        // };

        // let current = self.current.required_size();

        // let preview = if self.preview.is_some() {
        //     let preview = self.preview.unwrap().lock();
        // // *preview.clone().lock().requied_size();
        // } else {
        //     (0, 0)
        // };

        Vec2::zero()

        // parent + current + preview
    }
}
