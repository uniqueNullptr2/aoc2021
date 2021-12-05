use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use itertools::Itertools;


#[aoc_generator(day1)]
pub fn generate_1(input: &str) -> Vec<u32> {
    input.lines().map(|s| u32::from_str(s.trim()).expect("parsing")).collect()
}
#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows::<(_,_)>()
        .filter(|(e,f)| f > e)
        .count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows::<(_,_,_,_)>()
        .filter(|(e,_,_,f)| f > e)
        .count()
}