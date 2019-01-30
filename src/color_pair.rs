use cursive::theme::{BaseColor, Color, ColorStyle, ColorType};
use std::fs::FileType;

pub enum DefaultColorPair {
    Red,
    Blue,
    Green,
    Cyan,
    White,
}

// There has to be a better way
impl DefaultColorPair {
    pub fn color_pair(&mut self) -> ColorPair {
        match self {
            DefaultColorPair::Red => {
                let default = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Red)),
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                );
                let highlight = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                    ColorType::Color(Color::Dark(BaseColor::Red)),
                );

                ColorPair { default, highlight }
            }
            DefaultColorPair::Blue => {
                let default = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Blue)),
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                );
                let highlight = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                    ColorType::Color(Color::Dark(BaseColor::Blue)),
                );

                ColorPair { default, highlight }
            }
            DefaultColorPair::Green => {
                let default = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Green)),
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                );
                let highlight = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                    ColorType::Color(Color::Dark(BaseColor::Green)),
                );

                ColorPair { default, highlight }
            }
            DefaultColorPair::Cyan => {
                let default = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Cyan)),
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                );
                let highlight = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                    ColorType::Color(Color::Dark(BaseColor::Cyan)),
                );

                ColorPair { default, highlight }
            }
            DefaultColorPair::White => {
                let default = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::White)),
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                );
                let highlight = ColorStyle::new(
                    ColorType::Color(Color::Dark(BaseColor::Black)),
                    ColorType::Color(Color::Dark(BaseColor::White)),
                );

                ColorPair { default, highlight }
            }
        }
    }
}

#[derive(Clone)]
pub struct ColorPair {
    pub default: ColorStyle,
    pub highlight: ColorStyle,
}

impl Default for ColorPair {
    fn default() -> Self {
        DefaultColorPair::White.color_pair()
    }
}

impl ColorPair {
    pub fn new(filetype: &FileType) -> Self {
        if filetype.is_dir() {
            DefaultColorPair::Blue.color_pair()
        } else if filetype.is_symlink() {
            DefaultColorPair::Cyan.color_pair()
        } else if filetype.is_file() {
            DefaultColorPair::White.color_pair()
        } else {
            ColorPair::default()
        }
    }
}
