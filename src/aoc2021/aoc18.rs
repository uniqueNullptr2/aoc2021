use std::cell::{RefCell};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum SnailNum {
    Pair{p: SnailPair},
    Num{n: u32}
}

impl SnailNum {
    pub fn n(&self) -> Option<u32> {
        if let SnailNum::Num{n} = self {
            Some(*n)
        } else {
            None
        }
    }
    pub fn p(&self) -> Option<&SnailPair> {
        if let SnailNum::Pair{p} = self {
            Some(p)
        } else {
            None
        }
    }
}
#[derive(Clone, Debug)]
pub struct SnailPair {
    l: Box<RefCell<SnailNum>>,
    r: Box<RefCell<SnailNum>>
}

impl Display for SnailNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailNum::Num { n } => write!(f, "{}",n),
            SnailNum::Pair { p } => write!(f, "{}", p)
        }
    }
}
impl Display for SnailPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.l.borrow(), self.r.borrow())
    }
}
impl SnailPair {
    pub fn parse(s: &str) -> SnailPair {
        let ss = &s[1..s.len()-1];
        let (ls, mut rs) = ss.split_at(SnailPair::find_middle(ss));
        rs = &rs[1..];
        let l = u32::from_str(ls).ok().map(|n| SnailNum::Num{n}).or_else(|| Some(SnailNum::Pair{p: SnailPair::parse(ls)})).unwrap();
        let r = u32::from_str(rs).ok().map(|n| SnailNum::Num{n}).or_else(|| Some(SnailNum::Pair{p: SnailPair::parse(rs)})).unwrap();
        SnailPair{l: Box::new(RefCell::new(l)),r: Box::new(RefCell::new(r))}
    }
    fn find_middle(s: &str) -> usize {
        let mut cc = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => cc += 1,
                ']' => cc -= 1,
                ',' if cc == 0 => return i,
                _ => ()
            }
        }
        0
    }

    pub fn add(l: SnailPair, r: SnailPair) -> SnailPair {
        SnailPair{l: Box::new(RefCell::new(SnailNum::Pair{p: l})),r: Box::new(RefCell::new(SnailNum::Pair{p: r}))}
    }

    pub fn magnitude(&self) -> u32 {
        let l = match &*self.l.borrow() {
            SnailNum::Pair { p } => p.magnitude(),
            SnailNum::Num { n } => *n
        };
        let r = match &*self.r.borrow() {
            SnailNum::Pair { p } => p.magnitude(),
            SnailNum::Num { n } => *n
        };
        3*l+2*r
    }

    pub fn reduce(&self) {
        // println!("{}\n", self);
        loop {
            let (_,_,e) = self.explode(0);
            if e {
                // println!("explosion");
                // println!("{}", self);
                continue;
            }

            if self.shift() {
                // println!("shift");
                // println!("{}", self);
                continue;
            }
            break;
        }
        // println!("\nend");
        // println!("{}", self);
    }
    // pass the result of a left explosion to the right subtree but left leaf first
    fn pass_right(&self, nn: u32) -> bool {
        match & mut *self.l.borrow_mut() {
            SnailNum::Num { n } => {
                *n += nn;
                return true;
            },
            SnailNum::Pair{p} => {
                if p.pass_right(nn) {
                    return true;
                }
            }
        }
        match & mut *self.r.borrow_mut() {
            SnailNum::Num { n } => {
                *n += nn;
                return true;
            },
            SnailNum::Pair{p} => {
                if p.pass_right(nn) {
                    return true;
                }
            }
        }
        false
    }

    // pass the result of a right explosion to the left subtree but right leaf first
    fn pass_left(&self, nn: u32) -> bool {
        match & mut *self.r.borrow_mut() {
            SnailNum::Num { n } => {
                *n += nn;
                return true;
            },
            SnailNum::Pair{p} => {
                if p.pass_left(nn) {
                    return true;
                }
            }
        }
        match & mut *self.l.borrow_mut() {
            SnailNum::Num { n } => {
                *n += nn;
                return true;
            },
            SnailNum::Pair{p} => {
                if p.pass_left(nn) {
                    return true;
                }
            }
        }
        false
    }

    fn explode(&self, d: usize) -> (u32,u32,bool) {
        if d >=3 {
            let (x,y,t) = if let SnailNum::Pair{p} = &*self.l.borrow() {
                (p.l.borrow().n().unwrap(),p.r.borrow().n().unwrap(),true)
            } else {
                (0,0,false)
            };
            if t {
                self.l.replace(SnailNum::Num{n: 0});
                match &mut *self.r.borrow_mut() {
                    SnailNum::Num{n} => {
                        *n += y;
                        return (x,0,true);
                    },
                    SnailNum::Pair{p} => {
                        if p.pass_right(y) {
                            return (x,0,true)
                        }
                    }
                }
                return (x,y,true);
            }
            let (x,y,t) = if let SnailNum::Pair{p} = &*self.r.borrow() {
                (p.l.borrow().n().unwrap(),p.r.borrow().n().unwrap(),true)
            } else {
                (0,0,false)
            };
            if t {
                self.r.replace(SnailNum::Num{n: 0});
                match &mut *self.l.borrow_mut() {
                    SnailNum::Num{n} => {
                        *n += x;
                        return (0,y,true);
                    },
                    SnailNum::Pair{p} => {
                        if p.pass_left(x) {
                            return (0,y,true)
                        }
                    }
                }
                return (x,y,true);
            }
            return (0,0,false);
        }
        if let SnailNum::Pair{p} = &*self.l.borrow() {
            let (x,y,e) = p.explode(d+1);
            if e {
                match &mut *self.r.borrow_mut() {
                    SnailNum::Pair{p} => {
                        if y != 0 && p.pass_right(y) {
                            return (x,0,e)
                        } else {
                            return (x,y,e)
                        }
                    },
                    SnailNum::Num{n} => {
                        *n += y;
                        return (x,0,e);
                    }
                }
            }
        }
        if let SnailNum::Pair{p} = &*self.r.borrow() {
            let (x,y,e) = p.explode(d+1);
            if e {
                match &mut *self.l.borrow_mut() {
                    SnailNum::Pair{p} => {
                        if x != 0 && p.pass_left(x) {
                            return (0,y,e)
                        } else {
                            return (x,y,e)
                        }
                    },
                    SnailNum::Num{n} => {
                        *n += x;
                        return (0,y,e);
                    }
                }
            }
        }

        (0,0,false)
    }

    fn shift(&self) -> bool {
        let (f, ls, s) = match &*self.l.borrow() {
            SnailNum::Pair { p } => (0,false, p.shift()),
            SnailNum::Num { n } if *n > 9 => (*n, true, true),
            _ => (0,false, false),
        };
        if ls {
            let l = (f as f32 / 2.0).floor() as u32;
            let r = (f as f32 / 2.0).ceil() as u32;
            self.l.replace(SnailNum::Pair{p: SnailPair{l: Box::new(RefCell::new(SnailNum::Num{n: l})),r: Box::new(RefCell::new(SnailNum::Num{n: r}))}});
            return true;
        } else if s {
            return s
        }

        let (f, rs, s) = match &*self.r.borrow() {
            SnailNum::Pair { p } => (0,false, p.shift()),
            SnailNum::Num { n } if *n > 9 => (*n, true, true),
            _ => (0,false, false),
        };
        if rs {
            let l = (f as f32 / 2.0).floor() as u32;
            let r = (f as f32 / 2.0).ceil() as u32;
            self.r.replace(SnailNum::Pair{p: SnailPair{l: Box::new(RefCell::new(SnailNum::Num{n: l})),r: Box::new(RefCell::new(SnailNum::Num{n: r}))}});
            return true;
        } else if s {
            return s
        }

        false
    }
}

pub fn input_generator(input: &str) -> Vec<SnailPair> {
    input.lines().map(|l| SnailPair::parse(l)).collect()
}


pub fn solve_part1(input: &[SnailPair]) -> u32 {
    let mut f = input[0].clone();

    for i in 1..input.len() {
        println!("  {}", f);
        println!("+ {}", input[i]);
        f = SnailPair::add(f, input[i].clone());
        f.reduce();
        println!("= {}\n", f);
    }
    f.magnitude()
}



pub fn solve_part2(input: &[SnailPair]) -> u32 {
    let mut max = 0;
    for i in 0..input.len()-1 {
        for e in i+1..input.len() {
            let f1 = SnailPair::add(input[i].clone(), input[e].clone());
            f1.reduce();
            let f2 = SnailPair::add(input[e].clone(), input[i].clone());
            f2.reduce();
            let ff1 = f1.magnitude();
            let ff2 = f2.magnitude();
            if ff1 > max {
                max = ff1;
            }
            if ff2 > max {
                max = ff2;
            }
        }
    }
    max
}