use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use itertools::Itertools;

pub enum Direction {
    UP(u32),
    DOWN(u32),
    FORWARD(u32),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|l| {
            let mut direction = l.trim().split(' ');
            let dir = direction.next().unwrap();
            let amount = u32::from_str(direction.next().unwrap()).unwrap();
            match dir {
                "forward" => Direction::FORWARD(amount),
                "down" => Direction::DOWN(amount),
                "up" => Direction::UP(amount),
                _ => Direction::FORWARD(0),
            }
        }).collect()
}
#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Direction>) -> u32 {
    let mut x = 0;
    let mut y = 0;
    for dir in input {
        match dir {
            Direction::FORWARD(n) => x += n,
            Direction::DOWN(n) => y += n,
            Direction::UP(n) => y -= n,
        }
    }
    x * y
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Direction>) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for dir in input {
        match dir {
            Direction::FORWARD(n) => {
                x += aim * n;
                y += n;
            },
            Direction::DOWN(n) => aim += n,
            Direction::UP(n) => aim -= n,
        }
    }
    x * y
}