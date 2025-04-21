use crate::day02::get_input::get_input;
use itertools;


fn get_windowed(report: &Vec<i32>) -> Vec<i32> {
    report.windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<i32>>()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let diffs = get_windowed(report);
    diffs.iter().cloned().all(|x| x > 0 && x <=3) || diffs.iter().cloned().all(|x| x < 0 && x >= -3)
}

fn is_safe_with_exception(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if is_safe(&new_report) {
            return true;
        }
    }

    false
}

pub fn part1(test: bool) -> i32 {
    let input = get_input(test);
    input.into_iter().filter(|report| is_safe(report)).count() as i32
}

pub fn part2(test: bool) -> i32 {
    let input = get_input(test);
    input.into_iter().filter(|report| is_safe_with_exception(report)).count() as i32
}
