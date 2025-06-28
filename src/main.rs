use aoc_2019::day3a::*;

pub fn main() {
    match solve_day3a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
