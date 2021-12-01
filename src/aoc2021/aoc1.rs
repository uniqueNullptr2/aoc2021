use aoc_runner_derive::{aoc};
use std::str::FromStr;
use itertools::Itertools;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut i = 0;
    for (e1, e2)in input.lines().map(|s| i32::from_str(s.trim()).expect("parsing")).tuple_windows::<(_,_)>() {
        if e2 > e1 {
            i += 1;
        }
    }
    i
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut i = 0;
    for (e1, e2) in input
                .lines()
                .map(|s| i32::from_str(s.trim()).expect("parsing"))
                .tuple_windows::<(_,_,_)>()
                .map(|(e1, e2, e3)| e1+e2+e3)
                .tuple_windows::<(_,_)>() {
        if e2 > e1 {
            i += 1;
        }
    }
    i
}