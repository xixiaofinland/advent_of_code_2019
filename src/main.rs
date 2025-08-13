use aoc_2019::day6b::*;

pub fn main() {
    match solve_day6b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
