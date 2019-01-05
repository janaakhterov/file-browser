use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::ColorStyle;
use std::fs::DirEntry;
use config::Config;
use std::os::unix::fs::PermissionsExt;
use std::ops::BitAnd;

pub struct ColorPair {
    pub regular: ColorStyle,
    pub highlight: ColorStyle,
}

impl Default  for ColorPair {
    fn default() -> Self {
        ColorPair {
            regular: ColorStyle::primary(),
            highlight: ColorStyle::highlight(),
        }
    }
}

impl ColorPair {
    pub fn new(entry: &DirEntry, settings: &mut Config) -> ColorPair {
        let meta = entry.metadata().unwrap();
        if meta.is_dir() {
            return ColorPair {
                regular: ColorStyle::new(
                    Color::Dark(BaseColor::Blue),
                    Color::Dark(BaseColor::Black)),
                highlight: ColorStyle::new(
                    Color::Dark(BaseColor::Black),
                    Color::Dark(BaseColor::Blue))
            };
        } else if meta.is_file() {
            if meta.permissions().mode().bitand(1) == 1 {
                return ColorPair {
                    regular: ColorStyle::new(
                        Color::Dark(BaseColor::Green),
                        Color::Dark(BaseColor::Black)),
                    highlight: ColorStyle::new(
                        Color::Dark(BaseColor::Black),
                        Color::Dark(BaseColor::Green))
                };
            }

            let ext = entry.path();
            let ext = ext.extension();
            if ext.is_none() {
                return ColorPair::default();
            }

            let ext = ext.unwrap().to_str();
            if let Some(ext) = ext {
                match settings.get_str(&ext) {
                    Ok(s) => {
                        match Color::parse(&s) {
                            Some(color) => return ColorPair {
                                regular: ColorStyle::new(color,
                                    Color::Dark(BaseColor::Black)),
                                highlight: ColorStyle::new(
                                    Color::Dark(BaseColor::Black),
                                    color)
                            },
                            None => {},
                        }
                    }
                    Err(_) => {},
                }
            }
            return ColorPair {
                regular: ColorStyle::new(
                             Color::Dark(BaseColor::White),
                             Color::Dark(BaseColor::Black)),
                highlight: ColorStyle::new(
                             Color::Dark(BaseColor::Black),
                             Color::Dark(BaseColor::White))
            }
        } else {
            return ColorPair::default();
        }
    }
}
