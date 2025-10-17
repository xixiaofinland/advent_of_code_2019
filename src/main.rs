use aoc_2019::day10b::solve_day10b;

pub fn main() {
    match solve_day10b() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
