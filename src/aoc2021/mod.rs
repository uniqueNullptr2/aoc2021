use crate::runner::{AocRunner, AocResult};

pub mod aoc1;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;
pub mod aoc6;
pub mod aoc7;
pub mod aoc8;
pub mod aoc9;
pub mod aoc10;
pub mod aoc11;
pub mod aoc12;
pub mod aoc13;
pub mod aoc14;
pub mod aoc15;
pub mod aoc16;
pub mod aoc17;
pub mod aoc18;
pub mod aoc19;

pub mod aoc21;

pub mod aoc24;

pub struct AocRunner2021 {
    day: usize,
}

impl AocRunner2021 {
    pub fn new() -> Self {
        Self{day: 0}
    }
}
impl Default for AocRunner2021 {
    fn default() -> Self {
        Self::new()
    }
}
impl AocRunner for AocRunner2021 {
    fn day (&self) -> usize {
        self.day
    }

    fn set_day(&mut self, day: usize) {
        self.day = day;
    }

    fn run(&mut self) -> Option<AocResult>{
        std::fs::read_to_string(format!("input/2021/day{}.txt", self.day)).ok().map(|s| {
            match self.day {
                1 => {
                    Some(self.run_day("AOC 2021 Day01", &s,
                        aoc1::generate_1,
                        |input| aoc1::solve_part1(input),
                        |input| aoc1::solve_part2(input)))
                },
                2 => {
                    Some(self.run_day("AOC 2021 Day02", &s,
                        aoc2::input_generator,
                        |input| aoc2::solve_part1(input),
                        |input| aoc2::solve_part2(input)))
                },
                3 => {
                    Some(self.run_day("AOC 2021 Day03", &s,
                        aoc3::input_generator,
                        |input| aoc3::solve_part1(input),
                        |input| aoc3::solve_part2(input)))
                },
                4 => {
                    Some(self.run_day_mut("AOC 2021 Day04", &s,
                        aoc4::input_generator,
                        aoc4::solve_part1,
                        aoc4::solve_part2))
                },
                // 5 => {
                //     Some(self.run_day("AOC 2021 Day05", &s,
                //         aoc5::input_generator,
                //         |input| aoc5::solve_part1(input),
                //         |input| aoc5::solve_part2_arr(input)))
                // },
                6 => {
                    Some(self.run_day_mut("AOC 2021 Day06", &s,
                        aoc6::input_generator,
                        aoc6::solve_part1,
                        aoc6::solve_part2))
                },
                7 => {
                    Some(self.run_day("AOC 2021 Day07", &s,
                        aoc7::input_generator,
                        |input| aoc7::solve_part1(input),
                        |input| aoc7::solve_part2(input)))
                },
                8 => {
                    Some(self.run_day("AOC 2021 Day08", &s,
                        aoc8::input_generator,
                        |input| aoc8::solve_part1(input),
                        |input| aoc8::solve_part2(input)))
                },
                9 => {
                    Some(self.run_day("AOC 2021 Day09", &s,
                        aoc9::input_generator,
                        |input| aoc9::solve_part1(input),
                        |input| aoc9::solve_part2(input)))
                },
                10 => {
                    Some(self.run_day("AOC 2021 Day10", &s,
                        aoc10::input_generator,
                        |input| aoc10::solve_part1(input),
                        |input| aoc10::solve_part2(input)))
                },
                11 => {
                    Some(self.run_day_mut("AOC 2021 Day11", &s,
                        aoc11::input_generator,
                        |input| aoc11::solve_part1(input),
                        |input| aoc11::solve_part2(input)))
                },
                12 => {
                    Some(self.run_day("AOC 2021 Day12", &s,
                        aoc12::input_generator,
                        aoc12::solve_part1,
                        aoc12::solve_part2))
                },
                13 => {
                    Some(self.run_day_mut("AOC 2021 Day13", &s,
                        aoc13::input_generator,
                        aoc13::solve_part1,
                        aoc13::solve_part2))
                },
                14 => {
                    Some(self.run_day("AOC 2021 Day14", &s,
                        aoc14::input_generator,
                        aoc14::solve_part1,
                        aoc14::solve_part2))
                },
                15 => {
                    Some(self.run_day("AOC 2021 Day15", &s,
                        aoc15::input_generator,
                        aoc15::solve_part1,
                        aoc15::solve_part2))
                },
                16 => {
                    Some(self.run_day("AOC 2021 Day16", &s,
                        aoc16::input_generator,
                        aoc16::solve_part1,
                        aoc16::solve_part2))
                },
                17 => {
                    Some(self.run_day("AOC 2021 Day17", &s,
                        aoc17::input_generator,
                        aoc17::solve_part1,
                        aoc17::solve_part2))
                },
                18 => {
                    Some(self.run_day("AOC 2021 Day18", &s,
                        aoc18::input_generator,
                        |input| aoc18::solve_part1(&input),
                        |input| aoc18::solve_part2(&input)))
                },
                _ => None
            }
        }).flatten()
    }
}