use crate::AoCResult;
use std::fs;

#[derive(Debug, Copy, Clone)]
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
    direction: Direction,
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
                direction: *direction,
            },
            DOWN => Line {
                start: *p,
                end: Point::from((p.x, p.y - *distance as isize)),
                direction: *direction,
            },
            LEFT => Line {
                start: *p,
                end: Point::from((p.x - *distance as isize, p.y)),
                direction: *direction,
            },
            RIGHT => Line {
                start: *p,
                end: Point::from((p.x + *distance as isize, p.y)),
                direction: *direction,
            },
        }
    }
}

pub fn solve_day3a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day3a_simple.txt")?;
    let (line1, line2) = file.trim().split_once('\n').ok_or("can't split")?;

    let line1_iter = line1.split(',');
    let mut lines_one: Vec<Line> = vec![];
    let mut start_point = Point::from((0, 0));

    for operation in line1_iter {
        let line = Line::new(&start_point, &Move::from(operation));
        start_point = line.end;
        lines_one.push(line);
    }

    let line2_iter = line2.split(',');
    let mut lines_two: Vec<Line> = vec![];
    let mut start_point = Point::from((0, 0));

    for operation in line2_iter {
        let line = Line::new(&start_point, &Move::from(operation));
        start_point = line.end;
        lines_two.push(line);
    }
    eprintln!("gopro[8]: day3a.rs:119: lines_two={:#?}", lines_two);

    Ok(0)
}
