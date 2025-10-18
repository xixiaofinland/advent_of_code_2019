use aoc_2019::day12a::solve_day12a;

pub fn main() {
    match solve_day12a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
