#![allow(clippy::type_complexity)]
use std::{collections::HashMap};

use itertools::{Itertools};


pub fn input_generator(input: &str) -> (String,HashMap<(char,char),char>) {
    let mut map = HashMap::new();
    let mut lines = input.lines();
    let s = lines.next().unwrap().trim().to_owned();
    lines.next().unwrap();
    for line in lines {
        let mut split = line.split(" -> ");
        let mut cc =split.next().unwrap().trim().chars();
        map.insert((cc.next().unwrap(), cc.next().unwrap()), split.next().unwrap().trim().chars().next().unwrap());
    }
    (s, map)
}

pub fn solve_part1(input: &(String,HashMap<(char,char),char>)) -> usize {
    // let mut mem = HashMap::new();
    let s = &input.0;
    let map = &input.1;
    solve(s, 10, map)
    // let mut iter = s.chars().tuple_windows::<(_,_)>();
    // let mut counts = [0;26];
    // for c in s.chars() {
    //     counts[c as usize -65] += 1
    // }
    // while let Some((a,b)) = iter.next() {
    //     add_counts(&mut counts, &solve_recursive(0,10,a,b,&mut mem, map));
    // }

    // let min = counts.iter().filter(|a| **a != 0).min().unwrap();
    // let max = counts.iter().max().unwrap();
    // max - min
}
fn solve(s: &str, iterations: usize, map: &HashMap<(char,char), char>) -> usize {
    let mut counter = [[0;26];26];
    for (a,b) in s.chars().tuple_windows::<(_,_)>() {
        counter[a as usize -65][b as usize-65] += 1;
    }
    for _ in 0..iterations {
        let mut c2 = [[0;26];26];
        for (i,l) in counter.into_iter().enumerate() {
            for (e, n) in l.into_iter().enumerate() {
                if n > 0 {
                    let a = (i+65) as u8 as char;
                    let b = (e+65) as u8 as char;
                    let c = map.get(&(a,b)).unwrap();
                    let x = *c as usize - 65;
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
    let min = cout.iter().filter(|a| **a != 0).min().unwrap();
    let max = cout.iter().max().unwrap();
    max - min
}


pub fn solve_part2(input: &(String,HashMap<(char,char),char>)) -> usize {
    let s = &input.0;
    let map = &input.1;
    solve(s, 40, map)
}
