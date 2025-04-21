use crate::common;
fn get_input_internal(filename: &str) -> String {
    common::read_all(filename, "day03")
}

pub fn get_input(test: bool) -> String {
    if test {
        get_input_internal("test_input.txt")
    }
    else {
        get_input_internal("input.txt")
    }
}

