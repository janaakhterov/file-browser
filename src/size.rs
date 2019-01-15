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
        let mut s = String::new();
        if self.prefix.len() > 0 {
            s.push_str(self.prefix);
            s.push(' ');
        }
        s.push_str(&self.size.to_string());
        if self.suffix.len() > 0 {
            s.push(' ');
            s.push_str(&self.suffix);
        }
        s
    }
}
