use crate::day01::get_input::get_input;
use itertools;
pub fn part1(test: bool) -> i32 {
    let (v1, v2) = get_input(test);
    let sorted1 = itertools::sorted(v1.into_iter());
    let sorted2 = itertools::sorted(v2.into_iter());

    sorted1
        .zip(sorted2)
        .map(|(x, y)| (x - y).abs())
        .collect::<Vec<_>>()
        .into_iter().sum()
}

pub fn part2(test: bool) -> i32 {
    let (v1, v2) = get_input(test);

    let result = v1.iter().cloned().fold(0, |acc, el1| {
        println!("{}", el1);
        let count: i32 =  v2.iter().cloned().filter(|el2| el1 == *el2).count() as i32;
        acc + (count * el1)

    });
    result
}
