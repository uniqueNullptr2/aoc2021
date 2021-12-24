#![allow(unused)]

trait Die {
    fn roll(&mut self) -> usize;
}
struct DeterministicDie {
    num: usize,
}

impl DeterministicDie {
    pub fn new() -> Self {
        Self{num:1}
    }
}
impl Default for DeterministicDie {
    fn default() -> Self {
        DeterministicDie::new()
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        let t = self.num + self.num+1 + self.num +2;
        self.num += 3;
        t
    }
}

#[derive(Clone, Copy)]
pub struct Game {
    size: usize,
    p1: usize,
    p2: usize,
    p1_score: usize,
    p2_score: usize
}

impl Game {
    fn new(p1: usize, p2: usize) -> Self {
        Self{size: 10, p1, p2, p1_score: 0, p2_score: 0}
    }

    fn advance_p1(&mut self, steps:usize) -> bool {
        self.p1 = Game::new_score(self.p1, steps);
        self.p1_score += self.p1;
        self.p1_score >= 1000
    }

    fn new_score(old: usize, steps: usize) -> usize {
        let mut t = old + steps;
        while t > 10 {
            t -= 10;
        }
        t
    }
    fn advance_p2(&mut self, steps:usize) -> bool {
        self.p2 = Game::new_score(self.p2, steps);
        self.p2_score += self.p2;
        self.p2_score >= 1000
    }
    fn score(&self) -> usize {
        println!("p1: {}\np2: {}", self.p1_score, self.p2_score);
        self.p1_score*self.p2_score
    }
}
pub fn input_generator(input: &str) -> Game {
    let mut t = input.lines().map(|l| {
        let mut split = l.split(": ");
        split.next().unwrap();
        usize::from_str_radix(split.next().unwrap().trim(), 10).unwrap()
    });
    Game::new(t.next().unwrap(), t.next().unwrap())
}
pub fn solve_part1(input: &mut Game) -> usize {
    let mut die = DeterministicDie::new();
    let mut rolls = 0;
    loop {
        rolls += 3;
        if input.advance_p1(die.roll()) {
            return input.p2_score*rolls;
        }

        rolls += 3;
        if input.advance_p2(die.roll()) {
            return input.p1_score*rolls;
        }

    }
}

pub fn solve_part2(input: &mut Game) -> usize {
    0
}