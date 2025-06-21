use aoc_2020::day1a::*;

pub fn main() {
    match solve_day1a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
