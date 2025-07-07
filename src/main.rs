use aoc_2019::day5a::*;

pub fn main() {
    match solve_day5a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
