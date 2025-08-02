use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day6a() -> AoCResult<usize> {
    let file = File::open("data/input_day6a.txt")?;
    let reader = BufReader::new(file);

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        update_graph(line, &mut graph);
    }

    let mut sum = 0;
    let mut queue: VecDeque<(String, usize)> = VecDeque::new(); // (node, depth)
    queue.push_back(("COM".to_string(), 0));

    while let Some((node, depth)) = queue.pop_front() {
        sum += depth;
        if let Some(children) = graph.get(&node) {
            for child in children {
                queue.push_back((child.clone(), depth + 1));
            }
        }
    }

    Ok(sum)
}

fn update_graph(data: String, gragh: &mut HashMap<String, Vec<String>>) {
    let (k, v) = data.split_once(')').unwrap();
    gragh.entry(k.to_string()).or_default().push(v.to_string());
}
