use cursive::theme::ColorStyle;
use cursive::theme::Color;
use cursive::theme::BaseColor;

pub enum PalleteColor {
    File,
    FileHigh,
    Dir,
    DirHigh,
    Exec,
    ExecHigh,
}

pub struct Pallete {
    pub file: ColorStyle,
    pub file_high: ColorStyle,
    pub dir: ColorStyle,
    pub dir_high: ColorStyle,
    pub exec: ColorStyle,
    pub exec_high: ColorStyle,
}

impl Pallete {
    fn new() -> Self {
        let file = ColorStyle::new(
            Color::Dark(BaseColor::White),
            Color::Dark(BaseColor::Black),
        );

        let file_high = ColorStyle::new(
            file.back,
            file.front,
        );

        let dir = ColorStyle::new(
            Color::Dark(BaseColor::Blue),
            Color::Dark(BaseColor::Black),
        );

        let dir_high = ColorStyle::new(
            dir.back,
            dir.front,
        );

        let exec = ColorStyle::new(
            Color::Dark(BaseColor::Green),
            Color::Dark(BaseColor::Black),
        );

        let exec_high = ColorStyle::new(
            exec.back,
            exec.front,
        );

        Pallete {
            file,
            file_high,
            dir,
            dir_high,
            exec,
            exec_high,
        }
    }

    pub fn set_color(&mut self, which: PalleteColor, color: ColorStyle) {
        match which {
            File => self.file = color,
            FileHigh => self.file_high = color,
            Dir => self.dir = color,
            DirHigh => self.dir_high = color,
            Exec => self.exec = color,
            ExecHigh => self.exec_high = color,
        }
    }
}
