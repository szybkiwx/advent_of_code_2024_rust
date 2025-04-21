use crate::common;
fn get_input_internal(filename: &str) -> (Vec<i32>, Vec<i32>) {
    common::read_lines(filename, "day01")
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a: i32 = parts.next().unwrap().parse().unwrap();
            let b: i32 = parts.next().unwrap().parse().unwrap();
            (a , b )
        }).unzip()
}

pub fn get_input(test: bool) -> (Vec<i32>, Vec<i32>) {
    if test {
        get_input_internal("test_input.txt")
    }
    else {
        get_input_internal("input.txt")
    }
}

