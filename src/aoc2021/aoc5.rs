use std::collections::BTreeMap;
use std::collections::HashMap;
use std::str::FromStr;

type Uint = u16;

pub trait LineMap {
    fn add_line (&mut self, x1: Uint, y1: Uint, x2: Uint, y2: Uint) {
        let xs = Uint::min(x1, x2);
        let xe = Uint::max(x1, x2);
        let ys = Uint::min(y1, y2);
        let ye = Uint::max(y1, y2);
        for y in ys..=ye {
            for x in xs..=xe {
                self.add_point(x, y);
            }
        }
    }
    fn add_diagonal (&mut self, x1: Uint, y1: Uint, x2: Uint, y2: Uint) {
        let xd = if x1 < x2 {1} else {-1};
        let yd = if y1 < y2 {1} else {-1};
        let mut x: i32 = x1 as i32;
        let mut y: i32 = y1 as i32;
        loop {
            if x == x2 as i32 {
                self.add_point(x as Uint, y as Uint);
                break;
            }
            self.add_point(x as Uint, y as Uint);
            x += xd;
            y += yd;
        }
    }
    fn add_point(&mut self, x: Uint, y: Uint);
    fn count(&self) -> usize;
}

impl LineMap for Vec<Vec<Uint>>{
    fn add_point(&mut self, x: Uint, y: Uint) {
        loop {
            if y < self.len() as Uint {
                break;
            }
            self.push(vec!());
        }
        let line = &mut self[y as usize];
        loop {
            if x < line.len() as Uint {
                break;
            }
            line.push(0);
        }
        self[y as usize][x as usize] += 1;
    }
    fn count(&self) -> usize {
        let mut c = 0;
            for l in self {
                for p in l {
                    if p >= &2 {
                        c += 1;
                    }
                }
            }
        c
    }
}

impl LineMap for HashMap<(Uint,Uint), u16> {
    fn add_point(&mut self, x: Uint, y: Uint) {
        *self.entry((x,y)).or_insert(0) += 1;
    }
    fn count(&self) -> usize {
        self.iter().filter(|(_,e)| **e >= 2).count()
    }
}


impl LineMap for BTreeMap<(Uint,Uint), u16> {
    fn add_point(&mut self, x: Uint, y: Uint) {
        *self.entry((x,y)).or_insert(0) += 1;
    }
    fn count(&self) -> usize {
        self.iter().filter(|(_,e)| **e >= 2).count()
    }
}

impl LineMap for [[u16;1000]; 1000] {
    fn add_point(&mut self, x: Uint, y: Uint) {
        self[y as usize][x as usize] += 1;
    }
    fn count(&self) -> usize {
        self.iter().map(|l| l.iter().filter(|i| **i >= 2).count()).sum()
    }
}


pub fn input_generator(input: &str) -> Vec<(Uint,Uint,Uint,Uint)> {
    let v = input
                .lines()
                .map(|l| {
                    let mut nums = l.split(" -> ")
                                .map(|t| {
                                    let mut nums = t.split(',').map(|n| Uint::from_str(n).unwrap());
                                    (nums.next().unwrap(), nums.next().unwrap())
                                });
                    let n1 = nums.next().unwrap();
                    let n2 = nums.next().unwrap();
                    (n1.0, n1.1, n2.0, n2.1)
                }).collect();
    v
}


pub fn solve_part1(input: &[(Uint,Uint,Uint,Uint)]) -> usize {
    let mut map = [[0u16;1000]; 1000];
    for line in input {
        if line.0 == line.2 || line.1 == line.3 {
            map.add_line(line.0, line.1, line.2, line.3)
        }
    }
    map.count()
}


pub fn solve_part2_map(input: &[(Uint,Uint,Uint,Uint)]) -> usize {
    let mut map = HashMap::new();
    solve(input, &mut map)
}


pub fn solve_part2_vec(input: &[(Uint,Uint,Uint,Uint)]) -> usize {
    let mut map = vec!();
    solve(input, &mut map)
}


pub fn solve_part2_arr(input: &[(Uint,Uint,Uint,Uint)]) -> usize {
    let mut map = [[0u16;1000];1000];
    solve(input, &mut map)
}


pub fn solve_part2_btree(input: &[(Uint,Uint,Uint,Uint)]) -> usize {
    let mut map = BTreeMap::new();
    solve(input, &mut map)
}

fn solve<T: LineMap> (input: &[(Uint,Uint,Uint,Uint)], map: &mut T) -> usize {
    for line in input {
        if line.0 == line.2 || line.1 == line.3 {
            map.add_line(line.0, line.1, line.2, line.3)
        } else {
            map.add_diagonal(line.0, line.1, line.2, line.3)
        }
    }
    map.count()
}