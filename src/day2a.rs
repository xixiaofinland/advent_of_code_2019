use crate::AoCResult;
use std::fs;

#[derive(Debug)]
enum Operation {
    Add { lhs: usize, rhs: usize, dest: usize },
    Multiply { lhs: usize, rhs: usize, dest: usize },
    Halt,
}

impl Operation {
    fn parse(slice: &[usize]) -> Self {
        if slice.is_empty() {
            panic!("Empty operation slice");
        }

        match slice[0] {
            1 | 2 if slice.len() < 4 => {
                panic!("Incomplete operation for opcode {}: {:?}", slice[0], slice);
            }
            1 => Operation::Add {
                lhs: slice[1],
                rhs: slice[2],
                dest: slice[3],
            },
            2 => Operation::Multiply {
                lhs: slice[1],
                rhs: slice[2],
                dest: slice[3],
            },
            99 => Operation::Halt,
            _ => panic!("Unknown opcode: {}", slice[0]),
        }
    }
}

fn execute(program: &mut [usize], op: &Operation) {
    match *op {
        Operation::Add { lhs, rhs, dest } => {
            program[dest] = program[lhs] + program[rhs];
        }
        Operation::Multiply { lhs, rhs, dest } => {
            program[dest] = program[lhs] * program[rhs];
        }
        Operation::Halt => {}
    }
}

pub fn solve_day2a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day2a.txt")?;
    let mut program: Vec<usize> = file
        .trim_end()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()?;

    program[1] = 12;
    program[2] = 2;

    let mut index = 0;
    while index < program.len() {
        let op = Operation::parse(&program[index..]);
        if matches!(op, Operation::Halt) {
            break;
        }
        execute(&mut program, &op);
        index += 4;
    }

    Ok(program[0])
}
