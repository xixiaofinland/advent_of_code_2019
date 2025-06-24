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
