use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::AoCResult;

pub fn solve_day6a() -> AoCResult<usize> {
    let file = File::open("data/input_day6a_simple.txt")?;
    let reader = BufReader::new(file);

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        update_graph(line, &mut graph);
    }
    eprintln!("gopro[2]: day6a.rs:13: graph={:#?}", graph);

    Ok(0)
}

fn update_graph(data: String, gragh: &mut HashMap<String, Vec<String>>) {
    let (k, v) = data.split_once(')').unwrap();
    gragh.entry(k.to_string()).or_default().push(v.to_string());
}
