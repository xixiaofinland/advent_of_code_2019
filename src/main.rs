use aoc_2019::day1b::*;

pub fn main() {
    match solve_day1b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
