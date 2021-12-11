use std::{collections::VecDeque};

type Octo = i8;

pub fn input_generator(input: &str) -> Vec<Octo> {
    input.lines()
        .map(|l| {
            l.chars().map(|c| c.to_digit(10).unwrap() as Octo)
        })
        .flatten()
        .collect()
}

pub fn solve_part1(_vec: &mut [Octo]) -> usize {
    let mut c = 0;
    for _ in 0..100 {
        c += flash_octos(_vec);
    }
    c
}

pub fn solve_part2(_vec: &mut [Octo]) -> usize {
    let mut i = 0;
    let l = _vec.len();
    loop {
        if flash_octos( _vec) == l {
            break;
        }
        i += 1;
    }
    i+1
}

fn flash_octos(octos: &mut [Octo]) -> usize {
    let mut stack = VecDeque::new();
    let len = (octos.len() as f32).sqrt() as usize;
    for (i, octo) in octos.iter_mut().enumerate() {
        *octo += 1;
        if *octo == 10 {
            stack.push_back(i);
        }
    }
    let mut c = 0;
    while let Some(i) = stack.pop_back() {
        let octo = &mut octos[i];
        if *octo != 0{
            *octo += 1;
            if *octo > 9 {
                c += 1;
                *octo = 0;
                for i in get_octos(i, len).into_iter().filter_map(|f| f) {
                    stack.push_back(i);
                }
            }
        }
    }
    c
}

fn get_octos(i: usize, l: usize) -> [Option<usize>;8] {
    let x = i % l;
    let y = (i-x)/l;
    let ym1 = y.checked_sub(1).filter(|n| *n < l);
    let yp1 = y.checked_add(1).filter(|n| *n < l);
    let xm1 = x.checked_sub(1).filter(|n| *n < l);
    let xp1 = x.checked_add(1).filter(|n| *n < l);
    [
        ym1.zip(xm1).map(|(i,e)| i*l+e),
        ym1.map(|n|n*l+x),
        ym1.zip(xp1).map(|(i,e)| i*l+e),
        xm1.map(|n|y*l+n),
        xp1.map(|n|y*l+n),
        yp1.zip(xm1).map(|(i,e)| i*l+e),
        yp1.map(|n|n*l+x),
        yp1.zip(xp1).map(|(i,e)| i*l+e),
    ]
}