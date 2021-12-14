use std::fmt::Display;
use std::str::FromStr;

use bitmaps::Bitmap;

#[derive(Clone)]
pub struct BingoBoard{
    board: Bitmap<25>,
    finished: bool,
    numbers: [Option<u8>; 100]
}

impl Default for BingoBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl BingoBoard {
    pub fn new() -> Self {
        Self{board:Bitmap::<25>::new(), finished: false, numbers: [None;100]}
    }
    pub fn insert(&mut self, number: usize, i: usize) {
        self.numbers[number] = Some(i as u8);
    }
    pub fn set(&mut self, num: u8) -> bool{
        if let Some(x) = self.numbers[num as usize]{
            self.board.set(x as usize, true);
        }
        if self.check() {
            self.finished = true;
            true
        } else {
            false
        }
    }

    pub fn sum(&self) -> usize {
        self.numbers.iter().enumerate().filter_map(|(i,u)| {
            match u {
                Some(x) if !self.board.get(*x as usize) => Some(i),
                _ => None,
            }
        }).sum()
    }

    fn check_row(&self, row: usize) -> bool {
        self.board.get(5*row) &&
        self.board.get(5*row+1) &&
        self.board.get(5*row+2) &&
        self.board.get(5*row+3) &&
        self.board.get(5*row+4)
    }

    fn check_col(&self, col: usize) -> bool {
        self.board.get(col) &&
        self.board.get(5+col) &&
        self.board.get(5*2+col) &&
        self.board.get(5*3+col) &&
        self.board.get(5*4+col)
    }

    fn check(&self) -> bool {
        for row in 0..5 {
            if self.check_row(row) {
                return true;
            }
        }
        for col in 0..5 {
            if self.check_col(col) {
                return true;
            }
        }
        false
    }
    pub fn finished(&self) -> bool {
        self.finished
    }
}
impl Display for BingoBoard {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut t = self.numbers.iter().flatten().enumerate().map(|(n, b)| (n, *b as usize, self.board.get(*b as usize))).collect::<Vec<(usize, usize, bool)>>();
        t.sort_by(|a,b| a.1.cmp(&b.1));
        let mut s = String::new();
        for (n, i, b) in t {
            if i != 0 && i % 5 == 0 {
                s.push('\n');
            }
            s.push_str(&format!("{:>8?}", (n,b)));
        }
        write!(fmt, "{}", s)
    }
}

pub fn input_generator(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut lines = input.trim_end().lines();
    let r = lines.next().unwrap().split(',').map(|e| u8::from_str(e).unwrap()).collect::<Vec<u8>>();
    let mut boards = vec!();
    while let Some(_) = lines.next() {
        let mut bingo = BingoBoard::new();
        for i in 0..5 {
            let line = lines.next().unwrap();
            for (e,s) in line.split_whitespace().enumerate() {
                bingo.insert(usize::from_str(s).unwrap(), 5*i+e);
            }
        }
        boards.push(bingo);
    }
    (r, boards)
}

pub fn solve_part1(input: &mut (Vec<u8>, Vec<BingoBoard>)) -> usize {
    let mut boards = input.1.clone();
    for i in input.0.iter() {
        for b in boards.iter_mut() {
            if b.set(*i) {
                return b.sum() * *i as usize;
            }
        }
    }
    0
}

#[allow(clippy::unnecessary_filter_map)]
pub fn solve_part2(input: &mut (Vec<u8>, Vec<BingoBoard>)) -> usize {
    let mut boards = input.1.clone();
    let len = boards.len();
    let mut c = 0;
    for i in input.0.iter() {
        for b in boards.iter_mut().filter(|b| !b.finished()) {
            if b.set(*i) {
                c+=1;
                if c == len {
                    return b.sum() * *i as usize;
                }
            }
        }
    }
    0
}