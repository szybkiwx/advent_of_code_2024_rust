use crate::day03::get_input::get_input;
use itertools;
use itertools::Itertools;
use regex::Regex;
pub fn part1(test: bool) -> i32 {
    let input = get_input(test);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(&input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .map(|(a, b)| a * b)
        .sum()
}

pub fn part2(test: bool) -> i32 {
    let input = get_input(test);
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();

    let all_instructions: Vec<_> = re.captures_iter(&input).map(|c| {
        let (_, [s]) = c.extract();
        s
    }).collect();

    let mut ignore = false;
    let mut valid_instructions = Vec::new();
    for instruction in all_instructions {
        match instruction {
            "do()" => ignore = false,
            "don't()" => ignore = true,
            _ =>  if !ignore {
                valid_instructions.push(instruction);
            }
        }
    }

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    valid_instructions.iter().map(|instruction| {
        re.captures_iter(instruction).map(|c| {
            let (_, [a, b]) = c.extract();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        }).next().unwrap()
    }).map(|(a, b)| a * b).sum()
}
