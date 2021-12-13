#![allow(clippy::type_complexity)]
use std::str::FromStr;

use itertools::Itertools;


pub fn input_generator(input: &str) -> (Vec<(i32,i32)>, Vec<(Option<i32>,Option<i32>)>) {
    let mut points = vec!();
    let mut folds = vec!();
    for l in input.lines() {
        if l.starts_with("fold along y") {
            let s = l.trim();
            let i = s.find('=').unwrap();
            folds.push((None, Some(i32::from_str(&s[i+1..]).unwrap())));
        } else if l.starts_with("fold along x") {
            let s = l.trim();
            let i = s.find('=').unwrap();
            folds.push((Some(i32::from_str(&s[i+1..]).unwrap()),None));
        } else if l.trim() != "" {
            let mut split = l.trim().split(',');
            points.push((split.next().map(|s| i32::from_str(s).unwrap()).unwrap(), split.next().map(|s| i32::from_str(s).unwrap()).unwrap()))
        }

    }
    (points, folds)
}

pub fn solve_part1(input: &mut (Vec<(i32,i32)>, Vec<(Option<i32>,Option<i32>)>)) -> usize {
    let o = input.1[0];
    input.0.iter().filter_map(|(x,y)| {
        match o {
            (Some(fx), None) if *x > fx => {
                Some(fx - (*x - fx)).filter(|n| *n >= 0).zip(Some(*y))
            },
            (None, Some(fy)) if *y > fy => {
                Some(*x).zip(Some(fy - (*y - fy)).filter(|n| *n >= 0))
            },
            _ => Some((*x,*y))
        }
    }).unique().count()
}

pub fn solve_part2(input: &mut (Vec<(i32,i32)>, Vec<(Option<i32>,Option<i32>)>)) -> String {
    for o in &input.1 {
        input.0.iter_mut().for_each(|(x,y)| {
            match o {
                (Some(fx), None) if *x > *fx => {
                    *x = fx - (*x - fx);
                },
                (None, Some(fy)) if *y > *fy => {
                    *y = fy - (*y - fy);
                },
                _ => ()
            }
        });
    }
    let mut v: Vec<(i32, i32)> = input.0.iter().unique().copied().collect();
    v.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let mut lx = 0;
    let mut ly = 0;
    let mut s = String::new();
    for (i,e) in v {
        for _ in 0..e-ly {
            s.push('\n');
            lx = 0;
        }
        ly = e;
        for _ in 0..i-lx {
            s.push(' ');
        }
        lx = i+1;
        s.push('#');
    }
    // println!("{}", s);
    //TODO actually parse grid
    "RGZLBHFP".to_owned()
}
