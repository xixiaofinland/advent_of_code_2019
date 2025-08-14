use aoc_2019::day8a::*;


pub fn main() {
    match solve_day8a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
