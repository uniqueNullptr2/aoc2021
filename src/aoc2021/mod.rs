use crate::runner::{AocRunner, Parts};

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


pub struct AocRunner2021 {
    day: usize,
    part: Parts,
}

impl AocRunner2021 {
    pub fn new() -> Self {
        Self{day: 0, part: Parts::BOTH}
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
    fn part(&self) -> Parts {
        self.part
    }
    fn set_day(&mut self, day: usize) {
        self.day = day;
    }
    fn set_part(&mut self, part: Parts){
        self.part = part;
    }
    fn run(&mut self) {
        if let Ok(s) = std::fs::read_to_string(format!("input/2021/day{}.txt", self.day)) {
            match self.day {
                1 => {
                    self.run_day_with_gen(&s,
                        aoc1::generate_1,
                        |input| aoc1::solve_part1(input),
                        |input| aoc1::solve_part2(input));
                },
                2 => {
                    self.run_day_with_gen(&s,
                        aoc2::input_generator,
                        |input| aoc2::solve_part1(input),
                        |input| aoc2::solve_part2(input));
                },
                3 => {
                    self.run_day_with_gen(&s,
                        aoc3::input_generator,
                        |input| aoc3::solve_part1(input),
                        |input| aoc3::solve_part2(input));
                },
                4 => {
                    self.run_day_with_gen(&s,
                        aoc4::input_generator,
                        aoc4::solve_part1,
                        aoc4::solve_part2);
                },
                5 => {
                    self.run_day_with_gen(&s,
                        aoc5::input_generator,
                        |input| aoc5::solve_part1(input),
                        |input| aoc5::solve_part2_arr(input));
                },
                6 => {
                    self.run_day_mut_with_gen(&s,
                        aoc6::input_generator,
                        aoc6::solve_part1,
                        aoc6::solve_part2);
                },
                7 => {
                    self.run_day_with_gen(&s,
                        aoc7::input_generator,
                        aoc7::solve_part1,
                        aoc7::solve_part2);
                },
                8 => {
                    self.run_day_with_gen(&s,
                        aoc8::input_generator,
                        |input| aoc8::solve_part1(input),
                        |input| aoc8::solve_part2(input));
                },
                9 => {
                    self.run_day_with_gen(&s,
                        aoc9::input_generator,
                        |input| aoc9::solve_part1(input),
                        |input| aoc9::solve_part2(input));
                },
                10 => {
                    self.run_day_with_gen(&s,
                        aoc10::input_generator,
                        |input| aoc10::solve_part1(input),
                        |input| aoc10::solve_part2(input));
                },
                11 => {
                    self.run_day_mut_with_gen(&s,
                        aoc11::input_generator,
                        |input| aoc11::solve_part1(input),
                        |input| aoc11::solve_part2(input));
                },
                12 => {
                    self.run_day_with_gen(&s,
                        aoc12::input_generator,
                        aoc12::solve_part1,
                        aoc12::solve_part2);
                },
                13 => {
                    self.run_day_mut_with_gen(&s,
                        aoc13::input_generator,
                        aoc13::solve_part1,
                        aoc13::solve_part2);
                },
                x => println!("Day{} not implemented", x)
            }
        } else {
            println!("Inputfile for Day {} was not found", self.day);
        }
    }
}