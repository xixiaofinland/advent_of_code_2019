use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day1b() -> AoCResult<usize> {
    let file = File::open("data/input_day1a.txt")?;
    let reader = BufReader::new(file);

    let numbers: usize = reader
        .lines()
        .filter_map(|line| line.ok()?.trim().parse::<usize>().ok())
        .map(cal_fuel)
        .sum();

    Ok(numbers)
}

fn cal_fuel(n: usize) -> usize {
    match n / 3 {
        x if x <= 2 => 0,
        x => {
            let fuel = x - 2;
            fuel + cal_fuel(fuel)
        }
    }
}

// fn cal_fuel(n: usize) -> usize {
//     let mut cal = (n / 3).saturating_sub(2);
//     if cal <= 0 {
//         0
//     } else {
//         cal += cal_fuel(cal);
//         cal
//     }
// }
