pub struct ViewRatio {
    pub(crate) parent: usize,
    pub(crate) main: usize,
    pub(crate) child: usize,
}

impl Default for ViewRatio {
    fn default() -> Self {
        ViewRatio::new(1, 2, 1)
    }
}

impl ViewRatio {
    pub(crate) fn new(parent: usize, main: usize, child: usize) -> Self {
        ViewRatio {
            parent,
            main,
            child,
        }
    }

    pub(crate) fn sum(&self) -> usize {
        self.parent.saturating_add(self.main).saturating_add(self.child)
    }
}
