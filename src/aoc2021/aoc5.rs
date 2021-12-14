use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;


#[derive(Clone, Copy)]
pub struct Line{
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = if self.is_horizontal() {
            "h"
        } else if self.is_vertical() {
            "v"
        } else if self.is_diagonal_positive() {
            "np"
        } else if self.is_diagonal_negative() {
            "dn"
        } else {
            "OHNO"
        };
        write!(f, "({},{}) -> ({},{}) = {}", self.x1, self.y1, self.x2, self.y2, t)
    }
}
impl Line {
    fn new(x1: u16, y1: u16, x2: u16, y2: u16) -> Self{
        let (xx1, yy1) = (x1,y1).min((x2,y2));
        let (xx2, yy2) = (x1,y1).max((x2,y2));
        Self{x1: xx1, x2: xx2, y1: yy1, y2: yy2}
    }
    pub fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }
    pub fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }
    pub fn is_diagonal_positive(&self) -> bool {
        self.y2 > self.y1
    }
    pub fn is_diagonal_negative(&self) -> bool {
        self.y2 < self.y1
    }

    fn has_point(&self, x: u16, y: u16) -> bool{
        if self.is_horizontal() {
            y == self.y1 && self.x1 <= x && self.x2 >= x
        } else if self.is_vertical() {
            x == self.x1 && self.y1 <= y && self.y2 >= y
        } else {
            false
        }
    }

    pub fn intersect_hv(&self, o: &Self, set: &mut HashSet<(u16,u16)>) {
        let (x,y) = (o.x1, self.y1);
        if self.x1 <= x && self.x2 >= x && o.y1 <= y && o.y2 >= y {
            set.insert((x,y));
        }
    }

    pub fn intersect(&self, o: &Self, set: &mut HashSet<(u16,u16)>) {
        if self.is_horizontal() && o.is_vertical() {
            
        } else if self.is_vertical() && o.is_horizontal() {
            let (x,y) = (self.x1, o.y1);
            if self.has_point(x, y) && o.has_point(x, y) {
                set.insert((x,y));
            }
        }
    }
    pub fn overlap_h(&self, o: &Self, set: &mut HashSet<(u16,u16)>) {
        if self.y1 == o.y1 {
            let y = self.y1;
            let x1 = self.x1.max(o.x1);
            let x2 = self.x2.min(o.x2);
            for x in x1..=x2 {
                set.insert((x,y));
            }
        }
    }
    pub fn overlap_v(&self, o: &Self, set: &mut HashSet<(u16,u16)>) {
        if self.x1 == o.x1 {
            let x = self.x1;
            let y1 = self.y1.max(o.y1);
            let y2 = self.y2.min(o.y2);
            for y in y1..=y2 {
                set.insert((x,y));
            }
        }
    }
}

fn diff(a: u16, b: u16) -> u16 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn input_generator(input: &str) -> (Vec<Line>,Vec<Line>,Vec<Line>,Vec<Line>) {
    let mut horizontal: Vec<Line> = vec!();
    let mut vertical: Vec<Line> = vec!();
    let mut diagonal_positive: Vec<Line> = vec!();
    let mut diagonal_negative: Vec<Line> = vec!();
    for line in input
        .lines()
        .map(|l| {
            let mut nums = l.split(" -> ")
                        .map(|t| {
                            let mut nums = t.split(',').map(|n| u16::from_str(n).unwrap());
                            (nums.next().unwrap(), nums.next().unwrap())
                        });
            let n1 = nums.next().unwrap();
            let n2 = nums.next().unwrap();
            let l = Line::new(n1.0, n1.1, n2.0, n2.1);
            // println!("{}", l);
            l
        }) {
            if line.is_horizontal() {
                horizontal.push(line);
            } else if line.is_vertical() {
                vertical.push(line);
            } else if line.is_diagonal_positive() {
                diagonal_positive.push(line);
            } else if line.is_diagonal_negative() {
                diagonal_negative.push(line);
            } else {
                println!("OH NO");
            }
        }
    (horizontal, vertical, diagonal_positive, diagonal_negative)
}


pub fn solve_part1(input: &(Vec<Line>,Vec<Line>,Vec<Line>,Vec<Line>)) -> usize {
    let (h,v,_dp,_dn) = input;
    let mut set = HashSet::new();
    for hl in h {
        for vl in v {
            hl.intersect_hv(vl, &mut set);
        }
    }
    for i in 0..h.len()-1 {
        for e in i+1..h.len() {
            h[i].overlap_h(&h[e], &mut set);
        }
    }
    for i in 0..v.len()-1 {
        for e in i+1..v.len() {
            v[i].overlap_v(&v[e], &mut set);
        }
    }
    set.len()
}


pub fn solve_part2(input: &(Vec<Line>,Vec<Line>,Vec<Line>,Vec<Line>)) -> usize {
    0
}
