use std::str::FromStr;

pub enum TargetResult {
    Calculating,
    Hit,
    OvershotX,
    OvershotY,
    UndershotX,
}
pub struct Probe {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32
}
impl Probe {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self{dx, dy, x: 0, y: 0}
    }

    pub fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        if self.dx > 0 {
            self.dx -= 1;
        } else if self.dx < 0 {
            self.dx += 1;
        }
        self.dy -= 1;
    }
}

#[derive(Clone,Copy)]
pub struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32
}

impl Target {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self{x1,x2,y1,y2}
    }
    pub fn contains_probe(&self, p: &Probe) -> TargetResult {
        if p.dx == 0 && p.x < self.x1 {
            TargetResult::UndershotX
        } else if p.y > self.y2  {
            TargetResult::OvershotY
        } else if p.x > self.x2  {
            TargetResult::OvershotX
        } else if p.x >= self.x1 && p.x <= self.x2 && p.y >= self.y1 && p.y <= self.y2 {
            TargetResult::Hit
        } else {
            TargetResult::Calculating
        }
    }
}

pub fn input_generator(input: &str) -> Target {
    let mut space = input.split_whitespace();
    space.next().unwrap();
    space.next().unwrap();
    let mut x = space.next().unwrap();
    println!("{}", &x[2..x.len()-1]);
    let mut tmp = x[2..x.len()-1].split("..");
    let (x1, x2) = (tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap(),
        tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap());
    x = space.next().unwrap();
    tmp = x[2..].split("..");
    let (y1, y2) = (tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap(),
        tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap());

    let xx1 = x1.min(x2);
    let xx2 = x1.max(x2);
    let yy1 = y1.min(y2);
    let yy2 = y1.max(y2);
    println!("{},{},{},{}", xx1, xx2, yy1, yy2);
    Target::new(xx1,yy1,xx2,yy2)
}

pub fn solve_part1(input: &Target) -> u64 {
    
    0
}



pub fn solve_part2(_input: &Target) -> u64 {
    0
}
