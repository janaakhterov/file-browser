use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show_hidden: bool,
    pub ext: HashMap<String, ColorValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorValue {
    hex(String),
    #[serde(skip)]
    ncurses(i32),
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            show_hidden: false,
            ext: HashMap::new(),
        }
    }
}

impl Settings {
    pub fn initalize_colors(&mut self) {

    }
}
