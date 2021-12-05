use std::fmt::Display;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

pub struct LineMap{
    map: Vec<Vec<u32>>
}
impl LineMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self{map: vec!(vec!(0u32; width); height)}
    }
    pub fn add_line (&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let xs = usize::min(x1, x2);
        let xe = usize::max(x1, x2);
        let ys = usize::min(y1, y2);
        let ye = usize::max(y1, y2);
        for y in ys..=ye {
            for x in xs..=xe {
                self.add_point(x, y);
            }
        }
    }
    pub fn add_diagonal (&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let xd = if x1 < x2 {1} else {-1};
        let yd = if y1 < y2 {1} else {-1};
        let mut x: i32 = x1 as i32;
        let mut y: i32 = y1 as i32;
        loop {
            if x == x2 as i32 {
                self.add_point(x as usize, y as usize);
                break;
            }
            self.add_point(x as usize, y as usize);
            x += xd;
            y += yd;
        }
    }

    fn add_point(&mut self, x: usize, y: usize) {
        loop {
            if y < self.map.len() {
                break;
            }
            self.map.push(vec!());
        }
        let line = &mut self.map[y];
        loop {
            if x < line.len() {
                break;
            }
            line.push(0);
        }
        self.map[y][x] += 1;
    }
    pub fn count(&self) -> usize {
        let mut c = 0;
            for l in &self.map {
                for p in l {
                    if p >= &2 {
                        c += 1;
                    }
                }
            }
        c
    }
}

impl Display for LineMap {
fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    let v : Vec<String> =self.map.iter()
            .map(|l| l.iter().map(|n| if n == &0 {".".to_owned()} else {n.to_string()}).collect::<String>()).collect();
    write!(fmt, "{}", v.join("\n"));
    Ok(())
}
}
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<(usize,usize,usize,usize)> {
    let v = input
                .lines()
                .map(|l| {
                    let mut nums = l.split(" -> ")
                                .map(|t| {
                                    let mut nums = t.split(',').map(|n| usize::from_str(n).unwrap());
                                    (nums.next().unwrap(), nums.next().unwrap())
                                });
                    let n1 = nums.next().unwrap();
                    let n2 = nums.next().unwrap();
                    (n1.0, n1.1, n2.0, n2.1)
                }).collect();
    v
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<(usize,usize,usize,usize)>) -> usize {
    let mut map = LineMap::new(10, 10);
    for line in input {
        if line.0 == line.2 || line.1 == line.3 {
            map.add_line(line.0, line.1, line.2, line.3)
        }
    }
    map.count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<(usize,usize,usize,usize)>) -> usize {
    let mut map = LineMap::new(10, 10);
    for line in input {
        if line.0 == line.2 || line.1 == line.3 {
            map.add_line(line.0, line.1, line.2, line.3)
        } else {
            map.add_diagonal(line.0, line.1, line.2, line.3)
        }
    }
    map.count()
}