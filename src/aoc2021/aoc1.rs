use std::str::FromStr;
use itertools::Itertools;


pub fn generate_1(input: &str) -> Vec<u32> {
    input.lines().map(|s| u32::from_str(s.trim()).expect("parsing")).collect()
}

pub fn solve_part1(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows::<(_,_)>()
        .filter(|(e,f)| f > e)
        .count()
}

pub fn solve_part2(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows::<(_,_,_,_)>()
        .filter(|(e,_,_,f)| f > e)
        .count()
}