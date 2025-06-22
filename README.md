# Learnings
It records what I learned in building the solutions

# D1
## d1b

fn `cal_fuel()` using `match` and calculate `- 2` without `saturating_sub()`

# D2
## d2a
I used `struct` + `enum`, but pure `enum` is better for this data modeling
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
