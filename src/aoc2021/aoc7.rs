use std::str::FromStr;

pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| usize::from_str(s.trim()).unwrap())
        .collect::<Vec<usize>>()
}

pub fn solve_part1(vec: &[usize]) -> usize {
    let mut l = 0;
    let mut r = *vec.iter().max().unwrap();
    loop {
        let a = l + (r-l)/2;
        let b = a-1;
        let c = a+1;

        let aa: usize = vec.iter().map(|n| diff(*n,a)).sum();
        let bb: usize = vec.iter().map(|n| diff(*n,b)).sum();
        let cc: usize = vec.iter().map(|n| diff(*n,c)).sum();
        if aa < bb && aa < cc {
            return aa;
        } else if bb < aa {
            r = a;
        } else {
            l = a;
        }
    }
}

fn find_mean(vec: &[usize]) -> (usize, usize, usize) {
    let c = (vec.iter().sum::<usize>() as f64 / vec.len() as f64).round() as usize;
    (c-1, c, c+1)
}

pub fn solve_part2(vec: &[usize]) -> usize {
    let (a,b,c) = find_mean(vec);

    let aa: usize = vec
        .iter()
        .map(|n| {
            let m = diff(*n,a);
            m*(m+1)/2
        })
        .sum();
    let bb: usize = vec
        .iter()
        .map(|n| {
            let m = diff(*n,b);
            m*(m+1)/2
        })
        .sum();
    let cc: usize = vec
        .iter()
        .map(|n| {
            let m = diff(*n,c);
            m*(m+1)/2
        })
        .sum();
    aa.min(bb).min(cc)
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}