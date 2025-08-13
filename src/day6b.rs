use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day6b() -> AoCResult<usize> {
    let file = File::open("data/input_day6a.txt")?;
    let reader = BufReader::new(file);

    let mut graph: HashMap<String, String> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        update_graph(line, &mut graph);
    }

    let mut you_trace: HashMap<String, usize> = HashMap::new();
    let mut outer = "YOU";
    let mut distance = 0;

    while let Some(inner) = graph.get(outer) {
        you_trace.insert(inner.to_string(), distance);
        distance += 1;
        outer = inner;
    }

    let mut outer = "SAN";
    let mut distance = 0;

    while let Some(inner) = graph.get(outer) {
        if let Some(dis) = you_trace.get(inner) {
            return Ok(dis + distance);
        }
        distance += 1;
        outer = inner;
    }
    unreachable!()
}

fn update_graph(data: String, gragh: &mut HashMap<String, String>) {
    let (inner, outer) = data.split_once(')').unwrap();
    gragh.insert(outer.to_string(), inner.to_string());
}
