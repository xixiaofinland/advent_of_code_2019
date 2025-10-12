use crate::AoCResult;
use std::{collections::HashSet, fs};

pub fn solve_day10a() -> AoCResult<usize> {
    let input = fs::read_to_string("data/input_day10a.txt")?;

    let mut asteroids = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert((x as isize, y as isize));
            }
        }
    }

    let mut max_visible = 0;

    for &a in &asteroids {
        let mut visible_dirs = HashSet::new();

        for &b in &asteroids {
            if a == b {
                continue;
            }

            let (ax, ay) = a;
            let (bx, by) = b;
            let dx = bx - ax;
            let dy = by - ay;
            let g = gcd(dx.abs(), dy.abs());
            visible_dirs.insert((dx / g, dy / g));
        }

        max_visible = max_visible.max(visible_dirs.len());
    }

    Ok(max_visible)
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

