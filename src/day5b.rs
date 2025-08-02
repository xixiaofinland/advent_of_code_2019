use crate::AoCResult;
use std::fs;

#[derive(Debug)]
enum Operation {
    Input { dest: usize },
    Output { value: isize },
    Add { lhs: isize, rhs: isize, dest: usize },
    Multiply { lhs: isize, rhs: isize, dest: usize },
    JumpIfTrue { cond: isize, target: isize },
    JumpIfFalse { cond: isize, target: isize },
    LessThan { lhs: isize, rhs: isize, dest: usize },
    Equals { lhs: isize, rhs: isize, dest: usize },
    Halt,
}

fn get_param(program: &[isize], mode: isize, val: isize) -> isize {
    match mode {
        0 => program[val as usize],
        1 => val,
        _ => panic!("Unknown parameter mode: {}", mode),
    }
}

fn parse(program: &[isize], ip: usize) -> Operation {
    let instruction = program[ip];
    let opcode = instruction % 100;
    let mode1 = (instruction / 100) % 10;
    let mode2 = (instruction / 1000) % 10;
    // mode3 is always position mode in 2019 day 5, so we can skip it

    match opcode {
        1 => Operation::Add {
            lhs: get_param(program, mode1, program[ip + 1]),
            rhs: get_param(program, mode2, program[ip + 2]),
            dest: program[ip + 3] as usize,
        },
        2 => Operation::Multiply {
            lhs: get_param(program, mode1, program[ip + 1]),
            rhs: get_param(program, mode2, program[ip + 2]),
            dest: program[ip + 3] as usize,
        },
        3 => Operation::Input {
            dest: program[ip + 1] as usize,
        },
        4 => Operation::Output {
            value: get_param(program, mode1, program[ip + 1]),
        },
        5 => Operation::JumpIfTrue {
            cond: get_param(program, mode1, program[ip + 1]),
            target: get_param(program, mode2, program[ip + 2]),
        },
        6 => Operation::JumpIfFalse {
            cond: get_param(program, mode1, program[ip + 1]),
            target: get_param(program, mode2, program[ip + 2]),
        },
        7 => Operation::LessThan {
            lhs: get_param(program, mode1, program[ip + 1]),
            rhs: get_param(program, mode2, program[ip + 2]),
            dest: program[ip + 3] as usize,
        },
        8 => Operation::Equals {
            lhs: get_param(program, mode1, program[ip + 1]),
            rhs: get_param(program, mode2, program[ip + 2]),
            dest: program[ip + 3] as usize,
        },
        99 => Operation::Halt,
        _ => panic!("Unknown opcode: {}", opcode),
    }
}

pub fn solve_day5b() -> AoCResult<isize> {
    let file = fs::read_to_string("data/input_day5a.txt")?;
    let mut program: Vec<isize> = file.trim_end().split(',').map(str::parse).collect::<Result<_, _>>()?;
    let mut ip = 0;
    let mut last_output = 0;

    loop {
        match parse(&program, ip) {
            Operation::Add { lhs, rhs, dest } => {
                program[dest] = lhs + rhs;
                ip += 4;
            }
            Operation::Multiply { lhs, rhs, dest } => {
                program[dest] = lhs * rhs;
                ip += 4;
            }
            Operation::Input { dest } => {
                program[dest] = 5; // Diagnostic input for part 2
                ip += 2;
            }
            Operation::Output { value } => {
                println!("output: {}", value);
                last_output = value;
                ip += 2;
            }
            Operation::JumpIfTrue { cond, target } => {
                ip = if cond != 0 { target as usize } else { ip + 3 };
            }
            Operation::JumpIfFalse { cond, target } => {
                ip = if cond == 0 { target as usize } else { ip + 3 };
            }
            Operation::LessThan { lhs, rhs, dest } => {
                program[dest] = if lhs < rhs { 1 } else { 0 };
                ip += 4;
            }
            Operation::Equals { lhs, rhs, dest } => {
                program[dest] = if lhs == rhs { 1 } else { 0 };
                ip += 4;
            }
            Operation::Halt => break,
        }
    }

    Ok(last_output)
}
