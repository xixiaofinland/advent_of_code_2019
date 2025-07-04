use crate::AoCResult;

pub fn solve_day4b() -> AoCResult<usize> {
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

    let has_exact_pair = digits.windows(2).enumerate().any(|(i, w)| {
        w[0] == w[1]
            && (i == 0 || digits[i - 1] != w[0])
            && (i + 2 >= digits.len() || digits[i + 2] != w[0])
    });

    is_incremental && has_exact_pair
}
