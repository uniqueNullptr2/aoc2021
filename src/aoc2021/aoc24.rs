use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread, fmt::Display};

use itertools::Itertools;

#[derive(Clone)]
struct Digit{
    d: [i64;14],
    dset: [SetState;14],
    input: Vec<Instruction>,
}

#[derive(Clone, Copy)]
enum SetState {
    Set,
    Unset
}

impl SetState {
    fn val(&self) -> u8 {
        match self {
            SetState::Set => 1,
            SetState::Unset => 0,
        }
    }
}

impl Digit {
    fn new(d: [i64;14], input: Vec<Instruction>) -> Self {
        Self{d, dset: [SetState::Unset; 14],input}
    }
    fn tryout(&mut self, i: usize) {
        let g = self.d[i];
        for f in 0..9 {
            self.d[i] = f;
            let mut alu = ALU::new(self.d);
            if let Some(r) = alu.execute_all(&self.input).ok() {
                println!("{} -> {}", f, r);
            }
        }
    }
    fn set(&mut self, i: usize) {
        let mut min = i64::max_value();
        let mut mc = 0;
        let mut e = 0;
        for f in 0..9 {
            self.d[i] = f;
            let mut alu = ALU::new(self.d);
            if let Some(r) = alu.execute_all(&self.input).ok() {
                if r < min {
                    min = r;
                    mc = 0;
                    e = f;
                } else if r == min {
                    mc += 1;
                }
            }
        }
        if mc == 0 {
            self.dset[i] = SetState::Set;
            self.d[i] = e;
        } else {
            self.d[i] = 8;
        }
    }

    fn set_all(&mut self) {
        let mut i = 14;
        while i > 0 {
            self.set(i-1);
            i -= 1
        }
    }
    fn increment(&mut self) -> bool {
        let mut i = 13;
        loop {
            match self.dset[i] {
                SetState::Set if i == 0 => {
                    return false;
                },
                SetState::Set => {
                    i -= 1;
                    continue;
                },
                _ => ()
            }
            self.d[i] -= 1;
            if self.d[i] < 0 {
                self.d[i] = 8;
                i -= 1;
            } else {
                break;
            }
        }
        true
    }
    fn calc(&self) -> Option<i64> {
        let mut alu = ALU::new(self.d);
        alu.execute_all(&self.input).ok()
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}{}{}{}{}{}{}{}{}{}{}{}\n{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            self.d[0]+1,
            self.d[1]+1,
            self.d[2]+1,
            self.d[3]+1,
            self.d[4]+1,
            self.d[5]+1,
            self.d[6]+1,
            self.d[7]+1,
            self.d[8]+1,
            self.d[9]+1,
            self.d[10]+1,
            self.d[11]+1,
            self.d[12]+1,
            self.d[13]+1,
            self.dset[0].val(),
            self.dset[1].val(),
            self.dset[2].val(),
            self.dset[3].val(),
            self.dset[4].val(),
            self.dset[5].val(),
            self.dset[6].val(),
            self.dset[7].val(),
            self.dset[8].val(),
            self.dset[9].val(),
            self.dset[10].val(),
            self.dset[11].val(),
            self.dset[12].val(),
            self.dset[13].val(),)
    }
}
#[derive(Clone, Copy)]
pub enum Instruction {
    Inp(Reference),
    Add(Reference, Reference),
    Mul(Reference, Reference),
    Div(Reference, Reference),
    Mod(Reference, Reference),
    Eql(Reference, Reference),
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        let mut split = s.trim().split_whitespace();
        let instr = split.next().unwrap();
        match instr {
            "inp" => Instruction::Inp(Reference::parse(split.next().unwrap())),
            "add" => Instruction::Add(Reference::parse(split.next().unwrap()),Reference::parse(split.next().unwrap())),
            "mul" => Instruction::Mul(Reference::parse(split.next().unwrap()),Reference::parse(split.next().unwrap())),
            "div" => Instruction::Div(Reference::parse(split.next().unwrap()),Reference::parse(split.next().unwrap())),
            "mod" => Instruction::Mod(Reference::parse(split.next().unwrap()),Reference::parse(split.next().unwrap())),
            "eql" => Instruction::Eql(Reference::parse(split.next().unwrap()),Reference::parse(split.next().unwrap())),
            _ => panic!("impossibru")
        }
    }
}
#[derive(Clone, Copy)]
pub enum Reference{
    W,
    X,
    Y,
    Z,
    Num(i64)
}

type Result<T> = std::result::Result<T, usize>;

impl Reference {
    fn parse(s: &str) -> Reference {
        match s {
            "w" => Reference::W,
            "x" => Reference::X,
            "y" => Reference::Y,
            "z" => Reference::Z,
            x => Reference::Num(i64::from_str_radix(x, 10).unwrap()),
        }
    }
}
pub struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    buf: [i64;14],
    index: usize,
}

impl ALU {
    fn new(buf: [i64;14]) -> Self {
        Self{w: 0, x: 0, y: 0, z: 0, buf, index: 0}
    }

    fn getn(&self, r: &Reference) -> i64{
        match r {
            Reference::W => self.w,
            Reference::X => self.x,
            Reference::Y => self.y,
            Reference::Z => self.z,
            Reference::Num(n) => *n
        }
    }
    fn setn(&mut self, r: &Reference, n: i64) -> Result<()>{
        match r {
            Reference::W => self.w = n,
            Reference::X => self.x = n,
            Reference::Y => self.y = n,
            Reference::Z => self.z = n,
            Reference::Num(_) => return Err(1)
        }
        Ok(())
    }
    fn execute_instr(&mut self, instr: &Instruction) -> Result<()>{
        match instr {
            Instruction::Inp(r) => {
                let i = self.buf[self.index]+1;
                self.index += 1;
                self.setn(r, i)?
            },
            Instruction::Add(a, b) => {
                let i = self.getn(a) + self.getn(b);
                self.setn(a, i)?;
            },
            Instruction::Mul(a, b) => {
                let i = self.getn(a) * self.getn(b);
                self.setn(a, i)?;
            },
            Instruction::Div(a, b) => {
                let i = ((self.getn(a) as f64 )/(self.getn(b) as f64).floor()) as i64;
                self.setn(a, i)?;
            },
            Instruction::Mod(a, b) => {
                let i = self.getn(a).checked_rem(self.getn(b)).ok_or(2usize)?;
                self.setn(a, i)?;
            },
            Instruction::Eql(a, b) => {
                let i = (self.getn(a) == self.getn(b)) as i64;
                self.setn(a, i)?;
            },
        }
        Ok(())
    }

    fn execute_all(&mut self, instructions: &[Instruction]) -> Result<i64> {
        for inst in instructions {
            self.execute_instr(inst)?
        }
        Ok(self.z)
    }
}
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| Instruction::parse(l))
        .collect()
}

pub fn solve_part1(input: &[Instruction]) -> i64 {
    let mut d = Digit::new([8,8,8,8,8,8,8,8,8,8,8,8,8,8],input.to_owned());
    println!("{}", d);
    d.tryout(1);
    // d.set_all();
    // loop {
    //     // println!("{}\n", d);

    //     match d.calc() {
    //         Some(0) => {
    //             println!("{}", d);
    //             return 0;
    //         },
    //         Some(x) => {
    //             // println!("{} -> {}", d, x)
    //         }
    //         None => ()
    //     }
    //     if !d.increment() {
    //         break;
    //     }
    // }
    1337
}



pub fn solve_part2(_input: &[Instruction]) -> usize {
    0
}
