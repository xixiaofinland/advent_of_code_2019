use aoc_2019::day10a::solve_day10a;

pub fn main() {
    match solve_day10a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
