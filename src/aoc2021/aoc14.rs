#![allow(clippy::type_complexity)]
use std::{str::FromStr, collections::HashMap};

use itertools::{Itertools, MinMaxResult};


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
    let mut mem = HashMap::new();
    let s = &input.0;
    let map = &input.1;
    let mut iter = s.chars().tuple_windows::<(_,_)>();
    let mut counts = [0;26];
    for c in s.chars() {
        counts[c as usize -65] += 1
    }
    while let Some((a,b)) = iter.next() {
        add_counts(&mut counts, &solve_recursive(0,10,a,b,&mut mem, map));
    }

    let min = counts.iter().filter(|a| **a != 0).min().unwrap();
    let max = counts.iter().max().unwrap();
    max - min
}

fn solve_recursive(
    depth: usize,
    max_depth: usize,
    a: char,
    b: char,
    mem: &mut HashMap<(char,char,usize),[usize; 26]>,
    map: & HashMap<(char,char), char>) -> [usize;26] {
        let mut counts = [0;26];
        match mem.get(&(a,b,depth)) {
            Some(c) => add_counts(&mut counts, c),
            None if depth < max_depth => {
                let c = *map.get(&(a,b)).unwrap();
                counts[c as usize - 65] += 1;
                let cc1 = solve_recursive(depth+1, max_depth, a, c, mem, map);
                let cc2 = solve_recursive(depth+1, max_depth, c, b, mem, map);
                add_counts(&mut counts, &cc1);
                add_counts(&mut counts, &cc2);
                mem.insert((a,b,depth), counts);

            }
            _ => {
                // counts[a as usize - 65] += 1;
                // counts[b as usize - 65] += 1;
                // mem.insert((a,b,depth), counts);
            }
        }
        counts
    }

fn add_counts(a: &mut[usize; 26], b: &[usize;26]) {
    for (i,n) in b.iter().enumerate() {
        a[i] += *n;
    }
}
pub fn solve_part2(input: &(String,HashMap<(char,char),char>)) -> usize {
    let mut mem = HashMap::new();
    let s = &input.0;
    let map = &input.1;
    let mut iter = s.chars().tuple_windows::<(_,_)>();
    let mut counts = [0;26];
    for c in s.chars() {
        counts[c as usize -65] += 1
    }
    while let Some((a,b)) = iter.next() {
        add_counts(&mut counts, &solve_recursive(0,40,a,b,&mut mem, map));
    }

    let min = counts.iter().filter(|a| **a != 0).min().unwrap();
    let max = counts.iter().max().unwrap();
    max - min
}
