use aoc_2019::day8::*;


pub fn main() {
    match solve_day8b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
