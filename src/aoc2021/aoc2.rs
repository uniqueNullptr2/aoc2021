#[derive(Clone, Copy)]
pub enum Direction {
    UP(u32),
    DOWN(u32),
    FORWARD(u32),
}


pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .lines()
        .map(|l| {
            let s = l.trim();
            let dir = s.chars().next().unwrap();
            let amount = s.chars().rev().next().unwrap() as u32 -48;
            match dir {
                'f' => Direction::FORWARD(amount),
                'd' => Direction::DOWN(amount),
                'u' => Direction::UP(amount),
                _ => Direction::FORWARD(0),
            }
        }).collect()
}


pub fn solve_part1(input: &[Direction]) -> u32 {
    let (x,y) = input.iter().fold((0,0), |(x,y), d| {
        match d {
            Direction::FORWARD(n) => (x+n, y),
            Direction::DOWN(n) => (x,y+n),
            Direction::UP(n) => (x,y-n),
        }
    });
    x * y
}

pub fn solve_part2(input: &[Direction]) -> u32 {
    let (x,y,_) = input.iter().fold((0,0,0), |(x, y, aim), d| {
        match d {
            Direction::FORWARD(n) => (x+n*aim, y+n, aim),
            Direction::DOWN(n) => (x,y,aim+n),
            Direction::UP(n) => (x,y,aim-n),
        }
    });
    x * y
}