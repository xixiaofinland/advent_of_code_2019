# Learnings It records what I learned in building the solutions

# D1 ## d1b

fn `cal_fuel()` using `match` and calculate `- 2` without `saturating_sub()`

# D2 ## d2a
1. I used `struct` + `enum`, but invalid state appears: opera1_addr,
   opera2_addr, and desti_addr are meaningless for `Op::Halt` `enum` only is
   better for this data modeling.

Use struct + enum when "Common Metadata + Variant Detail"

Mine: ```rust struct Operation { op: Op, opera1_addr: usize, opera2_addr: usize,
desti_addr: usize, } ```

GPT's: ```rust enum Operation { Add { lhs: usize, rhs: usize, dest: usize },
Multiply { lhs: usize, rhs: usize, dest: usize }, Halt, } ```

# D3 ## d3a

```rust let min_distance = lines_one .iter() .flat_map(|l1| { lines_two .iter()
.filter_map(move |l2| get_intersection(l1, l2)) }) .filter(|p| *p !=
Point::default()) .map(|p| p.x.abs() as usize + p.y.abs() as usize) .min()
.unwrap_or(0); ```

v.s.

```rust for l1 in &lines_one { for l2 in &lines_two { if let Some(p) =
get_intersection(l1, l2) { if p.x != 0 || p.y != 0 { let distance = p.x.abs() as
usize + p.y.abs() as usize; if distance < min_distance { min_distance =
distance; } } } } } ```

- range construction, then `.contains()`

```rust (h.start.x.min(h.end.x)..=h.start.x.max(h.end.x)).contains(&x) ```

## d3b

Check `steps_to_point()`, it calculate the steps nicely!

```rust fn steps_to_point(lines: &[Line], target: Point) -> Option<usize> { let
mut steps = 0; for line in lines { let (range_x, range_y) = (
line.start.x.min(line.end.x)..=line.start.x.max(line.end.x),
line.start.y.min(line.end.y)..=line.start.y.max(line.end.y),); if
range_x.contains(&target.x) && range_y.contains(&target.y) { steps += (target.x
- line.start.x).abs() as usize + (target.y - line.start.y).abs() as usize;
return Some(steps); } else { steps += (line.end.x - line.start.x).abs() as usize
                + (line.end.y - line.start.y).abs() as usize; } } None } ```

# D4

## d4a

- `.to_digit(10)` for one char converting
- `.windows(n)` is cool


```rust fn found(num: usize) -> bool { let digits: Vec<usize> = num .to_string()
.chars() .map(|c| c.to_digit(10).unwrap() as usize) .collect();

    let is_incremental = digits.windows(2).all(|w| w[0] <= w[1]); let has_double
    = digits.windows(2).any(|w| w[0] == w[1]);

    is_incremental && has_double } ```

## d4b

- `.windows()` has no built-in access to previous or next window, so "at least
  one exact_pair" is better with `for_loop` rather than `iter.windows(2)`

```rust let has_exact_pair = (0..digits.len() - 1).any(|i| { digits[i] ==
digits[i + 1] && (i == 0 || digits[i - 1] != digits[i]) && (i + 2 >=
digits.len() || digits[i + 2] != digits[i]) }); ```

```rust let has_exact_pair = digits.windows(2).enumerate().any(|(i, w)| { w[0]
== w[1] && (i == 0 || digits[i - 1] != w[0]) && (i + 2 >= digits.len() ||
digits[i + 2] != w[0]) }); ```

## d6a

The `VecDeque` is smart for this BFS problem. It doesn't need to track level or
level nodes separately!

```rust let mut sum = 0; let mut queue: VecDeque<(String, usize)> =
VecDeque::new(); // (node, depth) queue.push_back(("COM".to_string(), 0));

while let Some((node, depth)) = queue.pop_front() { sum += depth; if let
Some(children) = graph.get(&node) { for child in children {
queue.push_back((child.clone(), depth + 1)); } } } ```

Even better: `VecDeque` uses `&str` as the key, also fits map.get()

```rust let mut sum = 0; let mut queue: VecDeque<(&str, usize)> =
VecDeque::new(); // (node, depth) queue.push_back(("COM", 0));

while let Some((node, depth)) = queue.pop_front() { sum += depth; if let
Some(children) = graph.get(node) { for child in children {
queue.push_back((child.as_str(), depth + 1)); } } } ```

# D8

## d8a

`min_by_key` usage

```rust let min_layer = layers .iter() .min_by_key(|l| l.iter().filter(|&&c| c
== '0').count()) .unwrap(); ```

## d8b

vec with capacity if we know excat size:

```rust let mut v = Vec::with_capacity(150); ```

compare my and its code :)

```rust (0..150).for_each(|i| { for layer in &layers { match layer[i] { '0' |
'1' => { v.push(layer[i]); break; } '2' => continue, _ => unreachable!(), } }
}); ```

```rust for i in 0..150 { let pixel = layers .iter() .map(|layer| layer[i])
.find(|&c| c != '2') .unwrap_or('2'); // Shouldn't happen, but be safe
final_image.push(pixel); } ```

## d10a

key problem: calculate if from asteroid A we can see asteroid B.

The solution has twe smart key points:

1. use gcd() to calculate the "unique directions"(`visible_dirs`) between A and
B.

2. `visible_dirs` is a hashmap so we automatically store only one "unique
directions" from multiple target asteroids.

This means: The “unique direction” trick comes from realizing visibility depends
only on direction, not distance — and GCD gives a perfect integer representation
of direction.

You might attempted to use line's slope(= dy / dx) to solve this problem, but
then you need to handle +/-(opposite directions) and floating numbers. So gcd()
is still the optimal solution.

Then, read the solution code to check how simple it is!

