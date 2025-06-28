use crate::AoCResult;
use std::fs;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'U' => Direction::UP,
            'D' => Direction::DOWN,
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => unreachable!(),
        }
    }
}

struct Move {
    direction: Direction,
    distance: usize,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        if input.len() != 2 {
            panic!("bad input: {:?}", input);
        }

        let mut chars = input.chars();

        Move {
            direction: Direction::from(chars.next().unwrap()),
            distance: chars.next().unwrap().to_digit(10).unwrap() as usize,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(
        p: &Point,
        Move {
            direction,
            distance,
        }: &Move,
    ) -> Self {
        use Direction::*;

        match direction {
            UP => Line {
                start: *p,
                end: Point::from((p.x, p.y + *distance as isize)),
            },
            DOWN => Line {
                start: *p,
                end: Point::from((p.x, p.y - *distance as isize)),
            },
            LEFT => Line {
                start: *p,
                end: Point::from((p.x - *distance as isize, p.y)),
            },
            RIGHT => Line {
                start: *p,
                end: Point::from((p.x + *distance as isize, p.y)),
            },
        }
    }
}

pub fn solve_day3a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day3a_simple.txt")?;
    let (line1, line2) = file.split_once('\n').ok_or("can't split")?;

    let line1_iter = line1.split(',');

    let mut lines_one: Vec<Line> = vec![];
    let mut start_point = Point::from((0, 0));

    for operation in line1_iter {
        lines_one.push(Line::new(&start_point, &Move::from(operation)));
    }

    eprintln!("gopro[7]: day3a.rs:97: lines_one={:#?}", lines_one);

    Ok(0)
}
