use itertools::Itertools;

use crate::AoCResult;
use std::fs;

#[derive(Debug)]
struct Operation {
    op: Op,
    opera1_addr: usize,
    opera2_addr: usize,
    desti_addr: usize,
}

impl Operation {
    fn new(elements: &[usize]) -> Self {
        match elements.len() {
            4 => Operation {
                op: Op::from(elements[0]),
                opera1_addr: elements[1],
                opera2_addr: elements[2],
                desti_addr: elements[3],
            },
            _ if elements[0] == 99 => Operation {
                op: Op::Halt,
                opera1_addr: 0,
                opera2_addr: 0,
                desti_addr: 0,
            },
            _ => panic!("bad input for converting Operation: {:#?}", elements),
        }
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
    Halt,
    Unknown,
}

impl From<usize> for Op {
    fn from(value: usize) -> Self {
        match value {
            1 => Op::Add,
            2 => Op::Multiply,
            99 => Op::Halt,
            _ => Op::Unknown,
        }
    }
}

pub fn solve_day2a() -> AoCResult<usize> {
    let file = fs::read_to_string("data/input_day2a.txt")?;
    let mut program: Vec<usize> = file
        .trim_end()
        .split(",")
        .map(str::parse::<usize>)
        .collect::<Result<_, _>>()?;

    program[1] = 12;
    program[2] = 2;

    let mut index = 0;
    while index + 3 <= program.len() {
        let operation = Operation::new(&program[index..=index + 3]);
        execute(&mut program, &operation);
        index += 4;
    }
    eprintln!("gopro[85]: day2a.rs:55: program={:#?}", program);

    Ok(0)
}

fn execute(program: &mut Vec<usize>, operation: &Operation) {
    use Op::*;

    match operation.op {
        Add => {
            program[operation.desti_addr] =
                program[operation.opera1_addr] + program[operation.opera2_addr];
        }
        Multiply => {
            program[operation.desti_addr] =
                program[operation.opera1_addr] * program[operation.opera2_addr];
        }
        Halt => return,
        _ => unreachable!(),
    }
}
