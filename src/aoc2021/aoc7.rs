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
    let mut l = usize::max_value();
    let e = vec.1;
    for i in 0..=e{
        let t = solve1(&vec.0, i);
        if t < l  {
            l = t;
        }
    }
    l
}


pub fn solve_part2(vec: &(Vec<(usize, usize)>, usize)) -> usize {
    let mut l = usize::max_value();
    let e = vec.1;
    for i in 0..=e as usize{
        let t = solve2(&vec.0, i, e);
        if t < l {
            l = t;
        }
    }
    l
}

fn solve1(vec: &[(usize,usize)], pos: usize) -> usize {
    vec.iter().map(|(i,n)| i.abs_diff(pos) * n).sum()
}

fn solve2(vec: &[(usize,usize)], pos: usize, max: usize) -> usize {
    let map = fuel(max);
    vec.iter().map(|(i, n)| map[i.abs_diff(pos)]*n).sum()
}
fn fuel(max: usize) -> Vec<usize> {
    let mut s = 0;
    let mut v = vec!();
    for i in 0..=max {
        s += i;
        v.push(s);
    }
    v
}