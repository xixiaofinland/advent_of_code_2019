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

pub fn solve_day8b() -> AoCResult<usize> {
    let input = fs::read_to_string("data/input_day8a.txt")?;
    let chars: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    let layers: Vec<&[char]> = chars.chunks(150).collect();

    let mut v = Vec::with_capacity(150);

    (0..150).for_each(|i| {
        for layer in &layers {
            match layer[i] {
                '0' | '1' => {
                    v.push(layer[i]);
                    break;
                }
                '2' => continue,
                _ => unreachable!(),
            }
        }
    });

    // idiomatic code;

    // for i in 0..150 {
    //     let pixel = layers
    //         .iter()
    //         .map(|layer| layer[i])
    //         .find(|&c| c != '2')
    //         .unwrap_or('2'); // Shouldn't happen, but be safe
    //     final_image.push(pixel);
    // }

    let s: String = v.iter().collect();

    for row in 0..6 {
        let start = row * 25;
        let end = start + 25;
        let line = &s[start..end];
        let rendered: String = line
            .chars()
            .map(|c| if c == '1' { '█' } else { ' ' })
            .collect();
        println!("{}", rendered);
    }

    Ok(0)
}
