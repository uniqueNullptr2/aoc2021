use std::{str::Lines};

use itertools::Itertools;
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
pub struct ALU<'a> {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    buf: Lines<'a>
}

impl<'a> ALU<'a> {
    fn new(input: &'a str) -> Self {
        Self{w: 0, x: 0, y: 0, z: 0, buf: input.lines()}
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
                let i = i64::from_str_radix(self.buf.next().unwrap().trim(), 10).unwrap();
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
    let mut c:i64 = 99999999999999;
    let mut m:i64 = 10000000000000;
    // let mut cc = 0;
    // loop {
    // for _ in 0..9 {
        if let Some(s) = make_input(c) {
            let mut alu = ALU::new(&s);
            match alu.execute_all(input) {
                Ok(0) => return c,
                Ok(x)=> {
                    println!("{}", x);
                    c -= x;
                },
                _ => ()
            }
            // cc += 1;
            // if cc == 100000 {
            //     println!("{}", c);
            //     cc = 0;
            // }
        }
    //     c -= 1;
    // }
    1337
}


fn make_input(n: i64) -> Option<String> {
    Some(n.to_string().chars().join("\n")).filter(|s| !s.contains('0'))
}

pub fn solve_part2(_input: &[Instruction]) -> usize {
    0
}

fn solve(v: [i64;9]) -> i64 {
    let mut x = 0;
    let mut w = 0;
    let mut z = 0;
    let mut y = 0;

    w = v[0];
    z = w+7;
    w = v[1];
    z *= 26;
    y = w+8;
    z *= y;

    w = v[2];
    x = 1+z;
    x = x%26;
    x += 13;
    


    z
}