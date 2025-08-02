use aoc_2019::day6a::*;

pub fn main() {
    match solve_day6a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
