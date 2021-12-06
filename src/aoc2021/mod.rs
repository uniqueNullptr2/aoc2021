use crate::runner::AocRunner;

pub mod aoc1;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;
pub mod aoc6;

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
        self.run_day(Some(|s: String| aoc1::generate_1(&s)),
                    |input: &mut Vec<u32>| aoc1::solve_part1(input),
                    |input: &mut Vec<u32>| aoc1::solve_part2(input));
    }
}