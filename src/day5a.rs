use crate::AoCResult;
use std::fs;

#[derive(Debug)]
enum Operation {
    Input { dest: usize },
    Output { dest: usize },
    Output10 { dest: usize },
    Add00 { lhs: usize, rhs: usize, dest: usize },
    Add10 { lhs: usize, rhs: isize, dest: usize },
    Add01 { lhs: isize, rhs: usize, dest: usize },
    Add11 { lhs: isize, rhs: isize, dest: usize },
    Multiply00 { lhs: usize, rhs: usize, dest: usize },
    Multiply10 { lhs: usize, rhs: isize, dest: usize },
    Multiply01 { lhs: isize, rhs: usize, dest: usize },
    Multiply11 { lhs: isize, rhs: isize, dest: usize },
    // Halt,
}

impl Operation {
    fn parse(slice: &[isize]) -> Self {
        if slice.is_empty() {
            panic!("Empty operation slice");
        }

        match slice[0] {
            3 => Operation::Input {
                dest: slice[1] as usize,
            },
            104 => Operation::Output10 {
                dest: slice[1] as usize,
            },
            4 => Operation::Output {
                dest: slice[1] as usize,
            },
            1 => Operation::Add00 {
                lhs: slice[1] as usize,
                rhs: slice[2] as usize,
                dest: slice[3] as usize,
            },
            101 => Operation::Add01 {
                lhs: slice[1],
                rhs: slice[2] as usize,
                dest: slice[3] as usize,
            },
            1001 => Operation::Add10 {
                lhs: slice[1] as usize,
                rhs: slice[2],
                dest: slice[3] as usize,
            },
            1101 => Operation::Add11 {
                lhs: slice[1],
                rhs: slice[2],
                dest: slice[3] as usize,
            },
            2 => Operation::Multiply00 {
                lhs: slice[1] as usize,
                rhs: slice[2] as usize,
                dest: slice[3] as usize,
            },
            102 => Operation::Multiply01 {
                lhs: slice[1],
                rhs: slice[2] as usize,
                dest: slice[3] as usize,
            },
            1002 => Operation::Multiply10 {
                lhs: slice[1] as usize,
                rhs: slice[2],
                dest: slice[3] as usize,
            },
            1102 => Operation::Multiply11 {
                lhs: slice[1],
                rhs: slice[2],
                dest: slice[3] as usize,
            },
            // 99 => Operation::Halt,
            _ => panic!("Unknown opcode: {}", slice[0]),
        }
    }
}

fn execute(program: &mut [isize], op: &Operation) -> usize {
    match *op {
        Operation::Input { dest } => {
            program[dest] = 1;
            return 2;
        }
        Operation::Output { dest } => {
            println!("result: {}", program[dest]);
            return 2;
        }
        Operation::Output10 { dest } => {
            println!("result 104: {}", dest);
            return 2;
        }
        Operation::Add00 { lhs, rhs, dest } => {
            program[dest] = program[lhs] + program[rhs];
            return 4;
        }
        Operation::Add01 { lhs, rhs, dest } => {
            program[dest] = lhs + program[rhs];
            return 4;
        }
        Operation::Add10 { lhs, rhs, dest } => {
            program[dest] = program[lhs] + rhs;
            return 4;
        }
        Operation::Add11 { lhs, rhs, dest } => {
            program[dest] = lhs + rhs;
            return 4;
        }
        Operation::Multiply00 { lhs, rhs, dest } => {
            program[dest] = program[lhs] * program[rhs];
            return 4;
        }
        Operation::Multiply01 { lhs, rhs, dest } => {
            program[dest] = lhs * program[rhs];
            return 4;
        }
        Operation::Multiply10 { lhs, rhs, dest } => {
            program[dest] = program[lhs] * rhs;
            return 4;
        }
        Operation::Multiply11 { lhs, rhs, dest } => {
            program[dest] = lhs * rhs;
            return 4;
        }
    }
}

pub fn solve_day5a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day5a.txt")?;
    let mut program: Vec<isize> = file
        .trim_end()
        .split(',')
        .map(str::parse::<isize>)
        .collect::<Result<_, _>>()?;

    let mut index = 0;
    while index < program.len() {
        let op = Operation::parse(&program[index..]);
        let consumed = execute(&mut program, &op);
        index += consumed;

        if matches!(op, Operation::Output { dest: _ }) {
            if program[index] == 99 {
                break;
            }
        }
    }

    Ok(0)
}
