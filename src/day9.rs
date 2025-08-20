use itertools::Update;

use crate::AoCResult;
use std::{collections::HashSet, fs, ops::Bound};

pub fn solve_day9a() -> AoCResult<usize> {
    let input = fs::read_to_string("data/input_day9a_simple.txt")?;

    let bound = input.lines().count();
    eprintln!("gopro[9]: {}:{}: bound={:#?}", file!(), line!(), bound);

    let mut grid = Vec::with_capacity(bound);
    for l in input.lines() {
        let line_data = l
            .chars()
            .map(|c| if c == '.' { 0 } else { 1 })
            .collect::<Vec<usize>>();
        grid.push(line_data);
    }

    let mut blocked = HashSet::new();
    let mut count = 0;

    scan_down(0, 0, bound, &grid, &mut blocked, &mut count);
    Ok(0)
}

fn scan_down(
    a: usize,
    b: usize,
    bound: usize,
    grid: &[Vec<usize>],
    blocked: &mut HashSet<(usize, usize)>,
    count: &mut usize,
) {
    let mut x = a + 1;

    while x < bound {
        let line = &grid[x];
        for (y, v) in line.iter().enumerate() {
            if *v == 1 && !blocked.contains(&(x, y)) {
                *count += 1;
                update_blocked_down(blocked, (a, b), (x, y), bound);
            }
        }

        x += 1;
    }
}

fn update_blocked_down(
    blocked: &mut HashSet<(usize, usize)>,
    (a, b): (usize, usize),
    (x, y): (usize, usize),
    bound: usize,
) {
    let d_x = x as isize - a as isize;
    let d_y = y as isize - b as isize;

    if d_x == 0 {
        for i in y..bound {
            blocked.insert((x, i));
        }
    }

    let mut next_x = x as isize + d_x;
    let mut next_y = y as isize + d_y;

    while next_x >= 0 && next_x < bound as isize && next_y >= 0 && next_y < bound as isize {
        blocked.insert((next_x as usize, next_y as usize));
        next_x = next_x + d_x;
        next_y = next_y + d_y;
    }
}
