use std::str::FromStr;
use std::collections::VecDeque;


pub fn input_generator(input: &str) -> VecDeque<usize> {
    let mut arr = [0usize;9];
    for n in input.split(',').map(|s| usize::from_str(s.trim()).unwrap()) {
        arr[n] += 1;
    }
    VecDeque::from(arr)
}


pub fn solve_part1(vec: &mut VecDeque<usize>) -> usize {
    run_days(vec, 80)
}


pub fn solve_part2(vec: &mut VecDeque<usize>) -> usize {
    run_days(vec, 256)
}

fn run_day(vec: &mut VecDeque<usize>) {
    let n = vec.pop_front().unwrap();
    *vec.get_mut(vec.len()-2).unwrap() += n;
    vec.push_back(n);
}

fn run_days(vec: &mut VecDeque<usize>, days: usize)  -> usize {
    for _ in 0..days {
        run_day(vec);
    }
    vec.iter().sum()
}