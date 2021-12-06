use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Copy, Clone)]
pub struct Fish {
    timer: u8,
}
impl Fish {
    pub fn new (timer: u8) -> Self {
        Self{timer}
    }
    pub fn spawn(&mut self) -> Option<Self> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(Self{timer: 8});
        }
        self.timer -= 1;
        None
    }
}
#[aoc_generator(day6)]
pub fn generator_6(input: &str) -> Vec<Fish> {
    input.split(',').map(|s| u8::from_str(s).unwrap()).map(|n| Fish::new(n)).collect()
}
#[aoc(day6, part1)]
pub fn solve_part1(input: &[Fish]) -> usize {
    let mut c = input.len();
    let mut map = HashMap::new();
    for f in input {
        c += count_helper(*f, 80, &mut map);
    }
    c
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Fish]) -> usize {
    let mut c = input.len();
    let mut map = HashMap::new();
    for f in input {
        c += count_helper(*f, 256, &mut map);
    }
    c
}


fn count_helper(mut fish: Fish, days: u16, map: &mut HashMap<(u8,u16),usize>) -> usize {
    let mut c = 0;
    for i in 1..=days {
        if let Some(f) = fish.spawn() {
            c += 1;
            if map.contains_key(&(f.timer, days-i)) {
                c += map.get(&(f.timer, days-i)).unwrap();
            } else {
                let t = f.timer;
                let cc = count_helper(f, days - i, map);
                map.insert((t, days-i), cc);
                c += cc;
            }
        }
    }
    c
}