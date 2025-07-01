# Learnings
It records what I learned in building the solutions

# D1
## d1b

fn `cal_fuel()` using `match` and calculate `- 2` without `saturating_sub()`

# D2
## d2a
1. I used `struct` + `enum`, but invalid state appears: opera1_addr, opera2_addr,
and desti_addr are meaningless for `Op::Halt`
`enum` only is better for this data modeling.

Use struct + enum when "Common Metadata + Variant Detail"

Mine:
```rust
struct Operation {
    op: Op,
    opera1_addr: usize,
    opera2_addr: usize,
    desti_addr: usize,
}
```

GPT's:
```rust
enum Operation {
    Add { lhs: usize, rhs: usize, dest: usize },
    Multiply { lhs: usize, rhs: usize, dest: usize },
    Halt,
}
```

# D3
## d3a

```rust
    let min_distance = lines_one
        .iter()
        .flat_map(|l1| {
            lines_two
                .iter()
                .filter_map(move |l2| get_intersection(l1, l2))
        })
        .filter(|p| *p != Point::default())
        .map(|p| p.x.abs() as usize + p.y.abs() as usize)
        .min()
        .unwrap_or(0);
```

v.s.

```rust
    for l1 in &lines_one {
        for l2 in &lines_two {
            if let Some(p) = get_intersection(l1, l2) {
                if p.x != 0 || p.y != 0 {
                    let distance = p.x.abs() as usize + p.y.abs() as usize;
                    if distance < min_distance {
                        min_distance = distance;
                    }
                }
            }
        }
    }
```
