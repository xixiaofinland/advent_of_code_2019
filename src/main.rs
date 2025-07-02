use aoc_2019::day3b::*;

pub fn main() {
    match solve_day3b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
