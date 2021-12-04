use std::fmt::Display;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Clone)]
pub struct BingoBoard{
    board: [[(u32,bool); 5]; 5]
}

impl BingoBoard {
    pub fn parse (input: Vec<Vec<u32>>) -> Self {
        Self{board:[[(input[0][0], false),(input[0][1], false),(input[0][2], false),(input[0][3], false),(input[0][4], false)],
                    [(input[1][0], false),(input[1][1], false),(input[1][2], false),(input[1][3], false),(input[1][4], false)],
                    [(input[2][0], false),(input[2][1], false),(input[2][2], false),(input[2][3], false),(input[2][4], false)],
                    [(input[3][0], false),(input[3][1], false),(input[3][2], false),(input[3][3], false),(input[3][4], false)],
                    [(input[4][0], false),(input[4][1], false),(input[4][2], false),(input[4][3], false),(input[4][4], false)]]}
    }
    pub fn set(&mut self, num: u32) {
        for i in 0..5 {
            for e in 0..5 {
                if self.board[i][e].0 == num {
                    self.board[i][e] = (self.board[i][e].0, true);
                }
            }
        }
    }
    pub fn sum(&self) -> u32 {
        let mut s = 0;
        for i in 0..5 {
            for e in 0..5 {
                if !self.board[i][e].1 {
                    s += self.board[i][e].0;
                }
            }
        }
        s
    }
    fn check_row(&self, row: usize) -> bool {
        self.board[row][0].1 && self.board[row][1].1 && self.board[row][2].1 && self.board[row][3].1 && self.board[row][4].1
    }
    fn check_col(&self, col: usize) -> bool {
        self.board[0][col].1 && self.board[1][col].1 && self.board[2][col].1 && self.board[3][col].1 && self.board[4][col].1
    }
    fn get_row(&self, row: usize) -> [u32;5] {
        [self.board[row][0].0, self.board[row][1].0,  self.board[row][2].0, self.board[row][3].0, self.board[row][4].0]
    }
    fn get_col(&self, col: usize) -> [u32;5] {
        [self.board[0][col].0, self.board[1][col].0, self.board[2][col].0, self.board[3][col].0, self.board[4][col].0]
    }
    pub fn check(&self) -> Option<[u32;5]> {
        for row in 0..5 {
            if self.check_row(row) {
                return Some(self.get_row(row));
            }
        }
        for col in 0..5 {
            if self.check_col(col) {
                return Some(self.get_col(col));
            }
        }
        None
    }
}
impl Display for BingoBoard {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.board)
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let v: Vec<&str> = input.lines().collect();
    let r = v[0].split(',').map(|e| u32::from_str(e).unwrap()).collect::<Vec<u32>>();
    let mut boards = vec!();
    let mut i = 1;
    loop {
        if i >= v.len() {
            break;
        }
        let bingo = v[i+1..i+6].iter().map(|r| r.split_whitespace().map(|c| u32::from_str(c).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
        boards.push(BingoBoard::parse(bingo));
        i += 6;
    }
    (r, boards)
}
#[aoc(day4, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<BingoBoard>)) -> u32 {
    let mut boards = input.1.clone();
    for i in input.0.iter() {
        for b in boards.iter_mut() {
            b.set(*i);
            if let Some(nums) = b.check() {
                return b.sum() * i;
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<BingoBoard>)) -> u32 {
    let mut boards = input.1.clone();
    let mut last_board = None;
    for i in input.0.iter() {
        boards = boards
            .into_iter()
            .map(|mut b| {b.set(*i); b})
            .filter_map(|b| {
                if let Some(_nums) = b.check() {
                    last_board = Some((b, *i));
                    None
                } else {
                    Some(b)
                }
            }).collect();
    }
    last_board.map(|(b, i)| b.sum() * i).unwrap()
}