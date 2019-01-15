pub struct SizeString {
    pub prefix: &'static str,
    pub size: Size,
    pub suffix: String,
}

pub enum Size {
    Usize(usize),
    Float(f64),
}

impl Size {
    pub(crate) fn new() -> Self {
        Size::Usize(0)
    }

    pub(crate) fn to_string(&self) -> String {
        match self {
            Size::Usize(v) => v.to_string(),
            Size::Float(v) => format!("{:.2}", v),
        }
    }
}

impl SizeString {
    pub(crate) fn new() -> Self {
        SizeString {
            prefix: "",
            size: Size::new(),
            suffix: String::new(),
        }
    }

    pub(crate) fn to_string(&self) -> String {
        format!("{} {:.2} {}", self.prefix, self.size.to_string(), self.suffix)
    }
}
