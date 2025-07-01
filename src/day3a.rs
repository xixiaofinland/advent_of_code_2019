use crate::AoCResult;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!("invalid direction: {}", c),
        }
    }
}

struct Move {
    direction: Direction,
    distance: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let (d, n) = s.split_at(1);
        Move {
            direction: d.chars().next().unwrap().into(),
            distance: n.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    direction: Direction,
}

impl Line {
    fn new(start: Point, mov: &Move) -> Self {
        let end = match mov.direction {
            Direction::Up => Point {
                x: start.x,
                y: start.y + mov.distance as isize,
            },
            Direction::Down => Point {
                x: start.x,
                y: start.y - mov.distance as isize,
            },
            Direction::Left => Point {
                x: start.x - mov.distance as isize,
                y: start.y,
            },
            Direction::Right => Point {
                x: start.x + mov.distance as isize,
                y: start.y,
            },
        };
        Line {
            start,
            end,
            direction: mov.direction,
        }
    }
}

pub fn solve_day3a() -> AoCResult<usize> {
    let input = fs::read_to_string("data/input_day3a_simple.txt")?;
    let (wire1, wire2) = input.trim().split_once('\n').ok_or("Can't split lines")?;

    let lines_one = build_lines(wire1);
    let lines_two = build_lines(wire2);

    let min_distance = lines_one
        .iter()
        .flat_map(|l1| {
            lines_two
                .iter()
                .filter_map(move |l2| get_intersection(l1, l2))
        })
        .filter(|p| *p != Point::default())
        .map(|p| p.x.abs() as usize + p.y.abs() as usize)
        .min()
        .unwrap_or(0);

    Ok(min_distance)
}

fn build_lines(line: &str) -> Vec<Line> {
    let mut point = Point::default();
    line.split(',')
        .map(|s| {
            let mv = Move::from(s);
            let line = Line::new(point, &mv);
            point = line.end;
            line
        })
        .collect()
}

fn get_intersection(l1: &Line, l2: &Line) -> Option<Point> {
    use Direction::*;

    match (l1.direction, l2.direction) {
        (Up | Down, Left | Right) => intersect(l1, l2),
        (Left | Right, Up | Down) => intersect(l2, l1),
        _ => None,
    }
}

fn intersect(v: &Line, h: &Line) -> Option<Point> {
    let x = v.start.x;
    let y = h.start.y;
    if (h.start.x.min(h.end.x)..=h.start.x.max(h.end.x)).contains(&x)
        && (v.start.y.min(v.end.y)..=v.start.y.max(v.end.y)).contains(&y)
    {
        Some(Point { x, y })
    } else {
        None
    }
}
