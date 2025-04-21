use crate::day04::get_input::get_input;
use itertools;

fn get_slice_horizontal(r: usize, c: usize, arr: Vec<Vec<char>>) -> String {
    if c + 3 < arr[0].len() {
        (0..4).map(|i| arr[r][c + i]).collect::<String>()
    }
    else {
        "".to_string()
    }
}

fn get_slice_vertical(r: usize, c: usize, arr: Vec<Vec<char>>) -> String {
    if r + 3 < arr.len() {
        (0..4).map(|i| arr[r+i][c]).collect::<String>()
    }
    else {
        "".to_string()
    }
}
fn get_slice_diagonal1(r: usize, c: usize, arr: Vec<Vec<char>>) -> String {
    if r + 3 < arr.len() && c + 3 < arr[0].len() {
        (0..4).map(|i| arr[r + i][c + i]).collect::<String>()
    }
    else {
        "".to_string()
    }
}

fn get_slice_diagonal2(r: usize, c: usize, arr: Vec<Vec<char>>) -> String {
    if r + 3 < arr.len() && c + 3 < arr[0].len() {
        (0..4).map(|i| arr[r+i][c+3-i]).collect::<String>()
    }
    else {
        "".to_string()
    }
}

fn get_slice_diagonal21(r: usize, c: usize, arr: Vec<Vec<char>>) -> String {
    (0..3).map(|i| arr[r + i][c + i]).collect::<String>()
}

fn get_slice_diagonal22(r: usize, c: usize, arr: Vec<Vec<char>>) -> String {
    (0..3).map(|i| arr[r+i][c+2-i]).collect::<String>()
}


pub fn part1(test: bool) -> i32 {
    let input = get_input(test);
    let col_count = input[0].len();
    let row_count = input.len();

    let mut result: i32 = 0;
    let mut interations: i32 = 0;

    for r in 0..row_count  {
        for c in 0..col_count  {
            let horiz = get_slice_horizontal(r, c, input.clone());
            let vert = get_slice_vertical(r, c, input.clone());
            let diag1 = get_slice_diagonal1(r, c, input.clone());
            let diag2 = get_slice_diagonal2(r, c, input.clone());

            if horiz == "XMAS" || horiz == "SAMX" {
                result += 1;
            }
            if vert == "XMAS" || vert == "SAMX" {
                result += 1;
            }
            if diag1 == "XMAS" || diag1 == "SAMX" {
                result += 1;
            }

            if diag2 == "XMAS" || diag2 == "SAMX" {
                result += 1;
            }

            interations +=1;

        }
    }

    result
}

pub fn part2(test: bool) -> i32 {
    let input = get_input(test);
    let col_count = input[0].len();
    let row_count = input.len();

    let mut result: i32 = 0;

    for r in 0..row_count - 2 {
        for c in 0..col_count - 2 {
            let diag1 = get_slice_diagonal21(r, c, input.clone());
            let diag2 = get_slice_diagonal22(r, c, input.clone());
            if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
                result += 1;
            }
        }
    }
    result
}
