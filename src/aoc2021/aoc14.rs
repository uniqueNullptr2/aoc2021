#![allow(clippy::type_complexity)]

use itertools::{Itertools, MinMaxResult};


pub fn input_generator(input: &str) -> (String,[[usize; 26]; 26]) {
    let mut map = [[0; 26]; 26];
    let mut lines = input.lines();
    let s = lines.next().unwrap().trim().to_owned();
    lines.next().unwrap();
    for line in lines {
        let mut cc =line.trim().chars();
        let a = cc.next().unwrap() as usize -65;
        let b = cc.next().unwrap() as usize -65;
        let mut ccc = cc.rev();
        map[a][b] = ccc.next().unwrap() as usize - 65;
    }
    (s, map)
}

pub fn solve_part1(input: &(String,[[usize; 26]; 26])) -> usize {
    let s = &input.0;
    let map = &input.1;
    solve(s, 10, map)
}
fn solve(s: &str, iterations: usize, map: &[[usize;26];26]) -> usize {
    let mut counter = [[0;26];26];
    for (a,b) in s.chars().tuple_windows::<(_,_)>() {
        counter[a as usize -65][b as usize-65] += 1;
    }
    for _ in 0..iterations {
        let mut c2 = [[0;26];26];
        for (i,l) in counter.into_iter().enumerate() {
            for (e, n) in l.into_iter().enumerate() {
                if n > 0 {
                    let x = map[i][e];
                    c2[i][x] += n;
                    c2[x][e] += n;
                }
            }
        }
        counter = c2;
    }
    let mut cout = [0;26];
    for (i,l) in counter.into_iter().enumerate() {
        for (_, n) in l.into_iter().enumerate() {
            cout[i] += n;
            // cout[e] += n;
        }
    }
    let l = s.chars().rev().next().unwrap();
    cout[l as usize -65] += 1;
    match cout.iter().filter(|a| **a != 0).minmax() {
        MinMaxResult::MinMax(min,max) => max-min,
        _ => 0
    }
}


pub fn solve_part2(input: &(String,[[usize; 26]; 26])) -> usize {
    let s = &input.0;
    let map = &input.1;
    solve(s, 40, map)
}
