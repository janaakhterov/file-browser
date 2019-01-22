use ncurses::*;

pub const DIR_COLOR: i16 = 1;
pub const DIR_SELECTED_COLOR: i16 = 2;

pub const FILE_COLOR: i16 = 3;
pub const FILE_SELECTED_COLOR: i16 = 4;

pub const LINK_COLOR: i16 = 5;
pub const LINK_SELECTED_COLOR: i16 = 6;

pub const EXEC_COLOR: i16 = 7;
pub const EXEC_SELECTED_COLOR: i16 = 8;

pub fn init_colors() {
    init_pair(DIR_COLOR, COLOR_BLUE, COLOR_BLACK);
    init_pair(DIR_SELECTED_COLOR, COLOR_BLACK, COLOR_BLUE);

    init_pair(FILE_COLOR, COLOR_WHITE, COLOR_BLACK);
    init_pair(FILE_SELECTED_COLOR, COLOR_BLACK, COLOR_WHITE);

    init_pair(LINK_COLOR, COLOR_CYAN, COLOR_BLACK);
    init_pair(LINK_SELECTED_COLOR, COLOR_BLACK, COLOR_CYAN);

    init_pair(EXEC_COLOR, COLOR_GREEN, COLOR_BLACK);
    init_pair(EXEC_SELECTED_COLOR, COLOR_BLACK, COLOR_GREEN);
}
