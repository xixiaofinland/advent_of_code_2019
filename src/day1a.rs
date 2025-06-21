use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day1a() -> AoCResult<usize> {
    let file = File::open("data/input_day1a.txt")?;
    let reader = BufReader::new(file);

    let numbers: usize = reader
        .lines()
        .filter_map(|line| line.ok()?.trim().parse::<usize>().ok())
        .map(|n| n / 3 - 2)
        .sum();

    Ok(numbers)
}
