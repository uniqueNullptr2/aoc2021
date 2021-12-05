use aoc_runner_derive::{aoc, aoc_generator};


#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|s| u32::from_str_radix(s, 2).unwrap()).collect()
}
const NUM: usize = 12;

#[aoc(day3, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let gamma = gamma(input);
    gamma * epsilon(gamma)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let co2 = filter_vec(input, false);
    let o2 = filter_vec(input, true);
    co2 * o2
}
fn filter_vec(input: &[u32], invers: bool) -> u32 {
    let mut v = input.to_owned();
    for i in (0..NUM).rev() {
        let mut t = most_common(&v, i);
        if invers {t = invert(t, NUM)}
        let f = 1 << i;
        v = v.into_iter().filter(|n| n & f == t & f).collect();
        if v.len() == 1 {
            return v[0]
        }
    }
    0
}
fn gamma(input: &[u32]) -> u32 {
    let mut n = 0;
    for i in (0..NUM).rev() {
        n += most_common(input, i);
    }
    n
}
fn epsilon(input: u32) -> u32 {
    invert(input, NUM)
}

fn invert(n: u32, positions: usize) -> u32 {
    let mut m = !n << (32 - positions);
    m >>= 32 - positions;
    m
}
fn most_common(v: &[u32], pos: usize) -> u32 {
    let mut c = 0;
    let t = 1 << pos;
    for i in v {
        if i & t == t {
            c += 1;
        }
    }
    if c >= v.len()-c {
        t
    } else {
        0
    }
}
