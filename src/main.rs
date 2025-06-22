use aoc_2019::day2a::*;

pub fn main() {
    match solve_day2a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
