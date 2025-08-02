use std::fs;

use crate::AoCResult;

pub fn solve_day6a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day6a.txt")?;
    let mut program: Vec<isize> = file
        .trim_end()
        .split(',')
        .map(str::parse::<isize>)
        .collect::<Result<_, _>>()?;
    Ok(0)
}
