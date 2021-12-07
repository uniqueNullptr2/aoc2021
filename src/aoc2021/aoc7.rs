use std::str::FromStr;
use std::collections::HashMap;
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
        let t = solve2(&vec.0, i);
        if t < l {
            l = t;
        }
    }
    l
}

fn solve1(vec: &Vec<(usize,usize)>, pos: usize) -> usize {
    vec.iter().map(|(i,n)| i.abs_diff(pos) * n).sum()
}

fn solve2(vec: &Vec<(usize,usize)>, pos: usize) -> usize {
    let mut map = HashMap::<usize, usize>::new();
    vec.iter().map(|(i, n)| fuel(i.abs_diff(pos), &mut map)*n).sum()
}
fn fuel(diff: usize, map: &mut HashMap<usize, usize>) -> usize {
    if map.contains_key(&diff) {
        *map.get(&diff).unwrap()
    } else if diff == 1 {
        1
    } else if diff == 0 {
        0
    } else {
        let f = fuel(diff-1, map) + diff;
        map.insert(diff, f);
        f
    }
}