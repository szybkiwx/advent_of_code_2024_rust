use crate::common;
fn get_input_internal(filename: &str) -> Vec<Vec<char>> {
    common::read_lines(filename, "day04")
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect()
}

pub fn get_input(test: bool) -> Vec<Vec<char>> {
    if test {
        get_input_internal("test_input.txt")
    }
    else {
        get_input_internal("input.txt")
    }
}

