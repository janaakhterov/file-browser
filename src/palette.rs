use cursive::theme::BaseColor;
use cursive::theme::Color;
use cursive::theme::ColorStyle;

pub enum PaletteColor {
    File,
    FileHigh,
    Dir,
    DirHigh,
    Exec,
    ExecHigh,
}

pub struct Palette {
    pub file: ColorStyle,
    pub file_high: ColorStyle,
    pub dir: ColorStyle,
    pub dir_high: ColorStyle,
    pub exec: ColorStyle,
    pub exec_high: ColorStyle,
}

impl Palette {
    pub fn new() -> Self {
        let file = ColorStyle::new(Color::Dark(BaseColor::White), Color::Dark(BaseColor::Black));

        let file_high = ColorStyle::new(file.back, file.front);

        let dir = ColorStyle::new(Color::Dark(BaseColor::Blue), Color::Dark(BaseColor::Black));

        let dir_high = ColorStyle::new(dir.back, dir.front);

        let exec = ColorStyle::new(Color::Dark(BaseColor::Green), Color::Dark(BaseColor::Black));

        let exec_high = ColorStyle::new(exec.back, exec.front);

        Palette {
            file,
            file_high,
            dir,
            dir_high,
            exec,
            exec_high,
        }
    }

    pub fn set_color(&mut self, which: PaletteColor, color: ColorStyle) {
        match which {
            PaletteColor::File => self.file = color,
            PaletteColor::FileHigh => self.file_high = color,
            PaletteColor::Dir => self.dir = color,
            PaletteColor::DirHigh => self.dir_high = color,
            PaletteColor::Exec => self.exec = color,
            PaletteColor::ExecHigh => self.exec_high = color,
        }
    }
}
