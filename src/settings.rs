use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub show_hidden: bool,
    pub ext: HashMap<String, String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            show_hidden: false,
            ext: HashMap::new(),
        }
    }
}
