use crate::AoCResult;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

// fixme: this code gives wrong answer despite the idea is on the right direction.
pub fn solve_day10b() -> AoCResult<usize> {
    let input = fs::read_to_string("data/input_day10a.txt")?;

    // Parse asteroid map
    let mut asteroids = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert((x as isize, y as isize));
            }
        }
    }

    // Find best station location (maximum visible asteroids)
    let mut best_location = (0, 0);
    let mut max_visible = 0;

    for &a in &asteroids {
        let mut visible_dirs = HashSet::new();

        for &b in &asteroids {
            if a == b {
                continue;
            }
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            let g = gcd(dx.abs(), dy.abs());
            visible_dirs.insert((dx / g, dy / g));
        }

        if visible_dirs.len() > max_visible {
            max_visible = visible_dirs.len();
            best_location = a;
        }
    }

    // Group asteroids by direction from the best location
    let mut directions: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for &b in &asteroids {
        if b == best_location {
            continue;
        }
        let dx = b.0 - best_location.0;
        let dy = b.1 - best_location.1;
        let g = gcd(dx.abs(), dy.abs());
        let dir = (dx / g, dy / g);
        directions.entry(dir).or_default().push(b);
    }

    // Sort each direction’s asteroid list by distance (nearest first)
    for asts in directions.values_mut() {
        asts.sort_by_key(|&(x, y)| {
            let dx = x - best_location.0;
            let dy = y - best_location.1;
            dx * dx + dy * dy
        });
    }

    // Sort directions by clockwise angle starting from up (0, -1)
    use std::f64::consts::TAU;
    let mut sorted_dirs: Vec<(isize, isize)> = directions.keys().copied().collect();
    sorted_dirs.sort_by(|&(dx1, dy1), &(dx2, dy2)| {
        let mut angle1 = (-dx1 as f64).atan2(dy1 as f64);
        let mut angle2 = (-dx2 as f64).atan2(dy2 as f64);
        if angle1 < 0.0 { angle1 += TAU; }
        if angle2 < 0.0 { angle2 += TAU; }
        angle1.partial_cmp(&angle2).unwrap()
    });

    // Vaporization simulation
    let mut vaporized = Vec::new();
    while !directions.is_empty() {
        let mut to_remove = vec![];

        for &dir in &sorted_dirs {
            if let Some(asts) = directions.get_mut(&dir) {
                if !asts.is_empty() {
                    let target = asts.remove(0);
                    vaporized.push(target);
                    if vaporized.len() == 200 {
                        println!("200th vaporized asteroid: {:?}", target);
                        return Ok((target.0 * 100 + target.1) as usize);
                    }
                }
                if asts.is_empty() {
                    to_remove.push(dir);
                }
            }
        }

        for dir in to_remove {
            directions.remove(&dir);
        }
    }

    Err("Less than 200 asteroids vaporized.".into())
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

