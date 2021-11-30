# aoc2021
Advent of Code 2021 solutions in rust


## Powered by cargo-aoc
https://crates.io/crates/cargo-aoc

### Commands
- download input: `cargo aoc input -d {day} -y {year}`
- run day with: `cargo aoc -d {day} -p {part}`
- run latest: `cargo aoc`
- bench: `cargo aoc bench`
### Examples

- generator
```rust
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
            (
                gift.next().unwrap(),
                gift.next().unwrap(),
                gift.next().unwrap(),
            )
        }).collect()
}
```

- solver
```rust
#[aoc(day2, part1)]
pub fn solve_part1(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));
            2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
        })
        .sum()
}
```