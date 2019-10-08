use std::{fs, process};
use std::io::Read;
use palette::{Srgb, Lab};


fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

pub fn has_dark_bg(col: Lab) -> bool {
    col.l < 50.0
}


pub struct ColorTheme {
    pub dark_bg: bool,
    pub fg1: String,
    pub fg2: String,
    pub bg1: String,
    pub bg01: String,
    pub bg2: String,
    pub bg3: String,
    pub bg4: String,
    pub keyword: String,
    pub builtin: String,
    pub constant: String,
    pub comment: String,
    pub func: String,
    pub string: String,
    pub typeface: String,
    pub warning: String,
    pub warning2: String,
    pub inv_builtin: String,
    pub inv_keyword: String,
    pub inv_type: String,
    pub inv_string: String,
    pub inv_warning: String,
    pub inv_warning2: String,
}


fn main() {
}
