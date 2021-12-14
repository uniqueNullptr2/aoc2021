use std::str::FromStr;

pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| usize::from_str(s.trim()).unwrap())
        .collect::<Vec<usize>>()
}

pub fn solve_part1(vec: &[usize]) -> usize {
    let mut i = 0;
    let mut last = 0;
    loop {
        let t: usize = vec
            .iter()
            .map(|n| diff(*n,i))
            .sum();
        if i != 0 && t > last {
            break;
        } else {
            last = t;
            i += 1;
        }
    }
    last
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