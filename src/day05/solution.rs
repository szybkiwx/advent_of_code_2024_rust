use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use crate::day05::get_input::get_input;
use itertools;
use itertools::Itertools;

fn is_correct(instructions: &HashMap<i32, HashSet<i32>>, pages: Vec<i32> ) -> bool {
    for (i, page) in pages.iter().enumerate() {
        for x in 0..i {
            if instructions.contains_key(page) && instructions[page].contains(&pages[x]) {
                return false;
            }
        }
    }
    true
}

pub fn part1(test: bool) -> i32 {
    let (instructions, updates) = get_input(test);
    let correct_pages = updates
        .into_iter()
        .filter(|pages|
            is_correct(&instructions, pages.clone())
        ).collect::<Vec<Vec<i32>>>();

    correct_pages.iter().map(|pages| pages[pages.len() / 2]).sum()
}

pub fn part2(test: bool) -> i32 {
    let (instructions, updates) = get_input(test);
    let incorrect_pages = updates
        .into_iter()
        .filter(|pages|
            !is_correct(&instructions, pages.clone())
        ).collect::<Vec<Vec<i32>>>();

    let corrected_pages = incorrect_pages
        .into_iter().map(|page|
            page
                .into_iter()
                .sorted_by(|a, b|
                    if instructions.contains_key(a) && instructions[a].contains(b) {
                        Ordering::Less
                    }
                    else {
                        Ordering::Greater
                    }
                )
                .collect::<Vec<_>>()
        )
        .collect::<Vec<Vec<i32>>>();

    corrected_pages.iter().map(|pages| pages[pages.len() / 2]).sum()
}
