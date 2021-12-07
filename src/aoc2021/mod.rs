use crate::runner::AocRunner;

pub mod aoc1;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;
pub mod aoc6;
pub mod aoc7;

pub struct AocRunner2021 {
    day: usize,
    part: usize,
}

impl AocRunner2021 {
    pub fn new (day: usize, part: usize) -> Self {
        Self{day, part}
    }
}
impl AocRunner for AocRunner2021 {
    fn run(& self) {
        self.run_day(Some(|s: String| aoc7::input_generator(&s)),
                    |input: &mut (Vec<(usize, usize)>, usize)| aoc7::solve_part1(input),
                    |input: &mut (Vec<(usize,usize)>, usize)| aoc7::solve_part2(input));
    }
}