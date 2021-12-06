use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use std::collections::VecDeque;


pub fn generator_6(input: &str) -> VecDeque<usize> {
    let mut arr = [0usize;9];
    for n in input.split(',').map(|s| usize::from_str(s).unwrap()) {
        arr[n] += 1;
    }
    VecDeque::from(arr)
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut vec = generator_6(input);
    run_days(&mut vec, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut vec = generator_6(input);
    run_days(&mut vec, 256)
}

fn run_day(vec: &mut VecDeque<usize>) {
    let n = vec.pop_front().unwrap();
    *vec.get_mut(vec.len()-2).unwrap() += n;
    vec.push_back(n);
}

fn run_days(vec: &mut VecDeque<usize>, days: usize)  -> usize {
    for _ in 0..days {
        run_day(vec);
    }
    vec.iter().sum()
}