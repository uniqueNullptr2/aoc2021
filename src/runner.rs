use std::{time::{Instant, Duration}, fmt::Display};


pub trait AocRunner {
    fn run(&self);

    fn run_day<A, G, T, F1, F2>(&self, gen: Option<G>, fn1: F1, fn2: F2)
    where T: Display,
    G: Fn(String) -> A,
    A: Clone,
    F1: FnMut(&'_ mut A) -> T,
    F2: FnMut(&'_ mut A) -> T
    {
        let s = std::fs::read_to_string("input/2021/day7.txt").unwrap();

        if let Some(genf) = gen {
            let (input, gen_time) = run_generator(s, genf);
            println!("day 1 - generator in {}", gen_time.as_nanos());

            let (sol1, solv1_time) = run_solver(&mut input.clone(), fn1);
            println!("day 1 - part 1 in {}\n\t=>{}", solv1_time.as_micros(), sol1);

            let (sol2, solv2_time) = run_solver(&mut input.clone(), fn2);
            println!("day 1 - part 2 in {}\n\t=>{}", solv2_time.as_micros(), sol2);
        } else {

        }
    }
}

fn run_generator<H, V: Fn(String) -> H>(s: String, func: V) -> (H, Duration) {
    let now = Instant::now();
    let input = func(s);
    (input, Instant::now()-now)
}

fn run_solver<A, T, F: FnMut(A) -> T>(input: A, mut func: F) -> (T, Duration) {
    let now = Instant::now();
    let result = func(input);
    (result, Instant::now()-now)
}
