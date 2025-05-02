use std::collections::{HashMap, HashSet};
use crate::common;
pub fn get_input_internal(filename: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let binding = common::read_all(filename, "day05").replace("\r\n", "\n");;
    let two_parts = binding
        .split("\n\n")
        .into_iter()
        .collect::<Vec<&str>>();

    let instructions = two_parts[0].to_string()
        .split("\n")
        .map(|line| {

            let binding = line.to_string();
            let binding = binding.trim();
            let tokens = binding.split("|").collect::<Vec<&str>>();
            println!("{:?}", tokens);
            let first = tokens[0].to_string().parse::<i32>().unwrap();
            let second = tokens[1].to_string().parse::<i32>().unwrap();
            (first, second)
        })
        .fold(
            HashMap::new(),
            |mut acc, (a, b)| {
                acc.entry(a).or_insert_with(HashSet::new).insert(b);
                acc
            },
        );
        //.collect::<HashMap<_,_>>();

    let pages = two_parts[1].to_string()
        .split("\n")
        .map(|line| { line.to_string().split(',').map(|x| x.parse::<i32>().unwrap()).collect() })
        .collect::<Vec<Vec<i32>>>();


    (instructions, pages)
}

pub fn get_input(test: bool) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    if test {
        get_input_internal("test_input.txt")
    }
    else {
        get_input_internal("input.txt")
    }
}

