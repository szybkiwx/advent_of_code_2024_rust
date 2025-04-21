use crate::common;
fn get_input_internal(filename: &str) -> Vec<Vec<i32>> {
    common::read_lines(filename, "day02")
        .iter()
        .map(|line| {
            let int_line= line.split_whitespace().into_iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
            int_line
        }).collect()
}

pub fn get_input(test: bool) -> Vec<Vec<i32>> {
    if test {
        get_input_internal("test_input.txt")
    }
    else {
        get_input_internal("input.txt")
    }
}

