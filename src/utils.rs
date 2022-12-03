use std::fs;

pub fn read_lines(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}
