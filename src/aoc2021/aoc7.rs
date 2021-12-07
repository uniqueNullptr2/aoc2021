use std::str::FromStr;

pub fn input_generator(input: &str) -> (Vec<(usize,usize)>, usize) {
    let v: Vec<usize> = input.split(',').map(|s| usize::from_str(s.trim()).unwrap()).collect();
    let max = *v.iter().max().unwrap();
    let mut r = vec!(0; max+1);
    for i in v {
        r[i] += 1;
    }
    (r.into_iter().enumerate().filter(|(_, e)| *e > 0).collect(), max)
}

pub fn solve_part1(vec: &(Vec<(usize, usize)>, usize)) -> usize {
    let e = vec.1;
    (0..=e)
        .map(|i| {
            vec.0
                .iter()
                .map(|(f, n)| diff(*f,i)*n)
                .sum()
        })
        .min()
        .unwrap()
}


pub fn solve_part2(vec: &(Vec<(usize, usize)>, usize)) -> usize {
    let e = vec.1;
    (0..=e)
        .map(|i| {
            vec.0
                .iter()
                .map(|(f, n)| {
                    let m = diff(*f,i);
                    (m*(m+1)/2)*n
                }).sum()
        })
        .min()
        .unwrap()
}

fn diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}