use aoc_2019::day4a::*;

pub fn main() {
    match solve_day4a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
