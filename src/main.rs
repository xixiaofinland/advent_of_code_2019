use aoc_2019::day1a::*;

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
