use crate::AoCResult;
use std::fs;

pub fn solve_day8a() -> AoCResult<usize> {
    let input = fs::read_to_string("data/input_day8a.txt")?;
    let chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    let layers: Vec<&[char]> = chars.chunks(150).collect();
    let min_layer = layers
        .iter()
        .min_by_key(|l| l.iter().filter(|&&c| c == '0').count())
        .unwrap();
    let one_count = min_layer.iter().filter(|&&c| c == '1').count();
    let two_count = min_layer.iter().filter(|&&c| c == '2').count();
    Ok(one_count * two_count)
}
