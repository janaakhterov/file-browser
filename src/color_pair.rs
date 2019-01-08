use config::Config;
use cursive::theme::{BaseColor, Color, ColorStyle};
use std::{fs::DirEntry, ops::BitAnd, os::unix::fs::PermissionsExt};
use std::collections::HashMap;
use failure::err_msg;
use failure::Error;

pub struct ColorPair {
    pub regular: ColorStyle,
    pub highlight: ColorStyle,
}

impl Default for ColorPair {
    fn default() -> Self {
        ColorPair {
            regular: ColorStyle::new(
                Color::Dark(BaseColor::White),
                Color::Dark(BaseColor::Black),
            ),
            highlight: ColorStyle::new(
                Color::Dark(BaseColor::Black),
                Color::Dark(BaseColor::White),
            ),
        }
    }
}

impl ColorPair {
    pub fn new(entry: &DirEntry, settings: &mut Config) -> Result<ColorPair, Error> {
        let meta = entry.metadata().unwrap();
        let filetype = entry.file_type()?;
        let colors = settings.get::<HashMap<String, String>>("ext");
        if colors.is_err() {
            return Err(err_msg("Failed to read colors"));
        }
        let colors = colors.unwrap();

        if filetype.is_dir() {
            Ok(ColorPair {
                regular: ColorStyle::new(
                    Color::Dark(BaseColor::Blue),
                    Color::Dark(BaseColor::Black),
                ),
                highlight: ColorStyle::new(
                    Color::Dark(BaseColor::Black),
                    Color::Dark(BaseColor::Blue),
                ),
            })
        } else if filetype.is_file() {
            if meta.permissions().mode().bitand(1) == 1 {
                return Ok(ColorPair {
                    regular: ColorStyle::new(
                        Color::Dark(BaseColor::Green),
                        Color::Dark(BaseColor::Black),
                    ),
                    highlight: ColorStyle::new(
                        Color::Dark(BaseColor::Black),
                        Color::Dark(BaseColor::Green),
                    ),
                });
            }

            let ext = entry.path();
            let ext = ext.extension();

            let ext = ext.ok_or_else(|| err_msg("Failed to unwrap ext"))?;
            let ext = ext.to_str().ok_or_else(|| err_msg("Failed to convert ext to str"))?;
            let color = colors.get(ext);

            if color.is_some() {
                let color = color.unwrap();
                let color = Color::parse(&color).ok_or_else(|| err_msg("Failed to parse color"))?;
                Ok(ColorPair {
                    regular: ColorStyle::new(color, Color::Dark(BaseColor::Black)),
                    highlight: ColorStyle::new(Color::Dark(BaseColor::Black), color),
                })
            } else {
                Ok(ColorPair {
                    regular: ColorStyle::new(
                        Color::Dark(BaseColor::White),
                        Color::Dark(BaseColor::Black),
                    ),
                    highlight: ColorStyle::new(
                        Color::Dark(BaseColor::Black),
                        Color::Dark(BaseColor::White),
                    ),
                })
            }
        } else if filetype.is_symlink() {
            Ok(ColorPair {
                regular: ColorStyle::new(
                    Color::Dark(BaseColor::Cyan),
                    Color::Dark(BaseColor::Black),
                ),
                highlight: ColorStyle::new(
                    Color::Dark(BaseColor::Black),
                    Color::Dark(BaseColor::Cyan),
                ),
            })
        } else {
            Err(err_msg("Unrecognized filetype"))
        }
    }
}
