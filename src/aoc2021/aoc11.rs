use std::collections::VecDeque;


pub fn input_generator(input: &str) -> [u8; 100] {
    input.lines()
        .map(|l| {
            l.chars().map(|c| c.to_digit(10).unwrap() as u8)
        })
        .flatten()
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

pub fn solve_part1(_vec: &mut [u8;100]) -> usize {
    let mut c = 0;
    for _ in 0..100 {
        c += flash_octos(_vec);
    }
    c
}

pub fn solve_part2(_vec: &mut [u8;100]) -> usize {
    let mut i = 0;
    loop {
        if flash_octos( _vec) == 100 {
            break;
        }
        i += 1;
    }
    i+1
}

fn flash_octos(octos: &mut [u8;100]) -> usize {
    let mut stack = VecDeque::<usize>::new();
    for (i,octo) in octos.iter_mut().enumerate() {
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
                for e in get_octos(i) {
                    if let Some(x) = e.map(|e| octos.get(e).map(|_| e)).flatten() {
                        stack.push_back(x);
                    }
                }
            }
        }
    }
    c
}

fn get_octos(i: usize) -> [Option<usize>;8] {
    let x = i % 10;
    let y = (i - x) / 10;
    let xm1 = x.checked_sub(1).filter(|n| *n < 10);
    let xp1 = x.checked_add(1).filter(|n| *n < 10);
    let ym1 = y.checked_sub(1).filter(|n| *n < 10);
    let yp1 = y.checked_add(1).filter(|n| *n < 10);
    [
        ym1.and(xm1).map(|_| i-11),
        ym1.map(|_| i-10),
        ym1.and(xp1).map(|_| i-9),
        xm1.map(|_| i-1),
        xp1.map(|_| i+1),
        yp1.and(xm1).map(|_| i+9),
        yp1.map(|_| i+10),
        yp1.and(xp1).map(|_| i+11),
    ]
}