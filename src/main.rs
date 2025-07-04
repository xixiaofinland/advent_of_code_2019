use aoc_2019::day4b::*;

pub fn main() {
    match solve_day4b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
