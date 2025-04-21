use std::fs::read_to_string;use std::path::Path;
use itertools::join;

pub fn read_lines(filename: &str, current_dir: &str) -> Vec<String> {
    let path = Path::new("src").join(current_dir).join(filename);

    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_all(filename: &str, current_dir: &str) -> String {
    let path = Path::new("src").join(current_dir).join(filename);
    read_to_string(path).unwrap()
}