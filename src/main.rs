use aoc_2019::day9::solve_day9a;

pub fn main() {
    match solve_day9a() {
        Ok(count) => {
            println!("Result: {}", count);
        }
        Err(e) => {
            eprintln!("Err: {}", e);
        }
    }
}
