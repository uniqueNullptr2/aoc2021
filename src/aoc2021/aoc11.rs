use std::collections::VecDeque;


pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|l| {
            l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>()
        })
        .collect()
}

pub fn solve_part1(_vec: &mut Vec<Vec<u8>>) -> usize {
    let mut c = 0;
    for _ in 0..100 {
        let mut flashy = vec!();
        for i in 0.._vec.len(){
            for e in 0.._vec[0].len() {
                _vec[i][e] += 1;
                if _vec[i][e] > 9 {
                    flashy.push((i,e))
                }
            }
        }
        for (i,e) in flashy {
            c += 1;
        }
    }
    c
}


pub fn solve_part2(_vec: &mut Vec<Vec<u8>>) -> usize {
    let m = _vec.len();
    let n = _vec[0].len();
    let mut i = 0;
    loop {
        if flash_octos( _vec) == m*n {
            break;
        }
        i += 1;
    }
    i+1
}

fn flash_octos(octos: &mut Vec<Vec<u8>>) -> usize {
    let mut stack = VecDeque::<(usize,usize)>::new();
    for i in 0..octos.len(){
        for e in 0..octos[0].len() {
            octos[i][e] += 1;
            if octos[i][e] == 10 {
                stack.push_back((i,e));
            }
        }
    }
    let mut c = 0;
    while let Some((i,e)) = stack.pop_back() {
        let octo = &mut octos[i][e];
        if *octo != 0{
            *octo += 1;
            if *octo > 9 {
                c += 1;
                *octo = 0;
                for (x,y) in get_octos(i, e) {
                    if octos.get(x).map(|l| l.get(y)).flatten().is_some() {
                        stack.push_back((x,y))
                    }
                }
            }
        }
    }
    c
}

const fn get_octos(i: usize, e: usize) -> [(usize,usize);8] {
    [
        (i-1,e-1),
        (i-1,e),
        (i-1,e+1),
        (i,e-1),
        (i,e+1),
        (i+1,e-1),
        (i+1,e),
        (i+1,e+1)
    ]
}