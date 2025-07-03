use std::collections::hash_set;

use crate::AoCResult;

pub fn solve_day4a() -> AoCResult<usize> {
    let mut count = 0;
    for num in 109165..576723 {
        if found(num) {
            println!("{num}");
            count += 1;
        }
    }
    Ok(count)
}

fn found(num: usize) -> bool {
    let digits: Vec<usize> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let is_incremental = digits.windows(2).all(|w| w[0] <= w[1]);
    let has_double = digits.windows(2).any(|w| w[0] == w[1]);

    is_incremental && has_double
}
