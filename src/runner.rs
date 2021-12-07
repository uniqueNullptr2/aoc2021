use std::{time::{Instant, Duration}, fmt::Display};


pub trait AocRunner {
    fn run(&self);

    fn day(&self) -> usize;
    fn part(&self) -> Parts;

    fn run_day_mut_with_gen<A, G, T, F1, F2>(&self, input: &str, gen: G, fn1: F1, fn2: F2)
    where T: Display,
    G: Fn(&str) -> A,
    A: Clone,
    F1: FnMut(&'_ mut A) -> T,
    F2: FnMut(&'_ mut A) -> T
    {
        let part = self.part();
        println!("Day {}", self.day());
        let (input, gen_time) = evaluate_function(input, gen);
        println!("    Generator in {}", gen_time.pretty());
        if part  == Parts::ONE || part == Parts::BOTH {
            let (sol1, solv1_time) = evaluate_function(&mut input.clone(), fn1);
            println!("    Part1 in {}\t=> {}",  solv1_time.pretty(), sol1)
        }
        if part  == Parts::TWO || part == Parts::BOTH {
            let (sol2, solv2_time) = evaluate_function(&mut input.clone(), fn2);
            println!("    Part2 in {}\t=> {}",  solv2_time.pretty(), sol2)
        }
    }

    fn run_day<A, G, T, F1, F2>(&self, input: &str, fn1: F1, fn2: F2)
    where T: Display,
    A: Clone,
    F1: Fn(&str) -> T,
    F2: Fn(&str) -> T
    {
        let part = self.part();
        println!("Day {}", self.day());
        if part  == Parts::ONE || part == Parts::BOTH {
            let (sol1, solv1_time) = evaluate_function(input, fn1);
            println!("    Part1 in {}\t=> {}",  solv1_time.pretty(), sol1)
        }
        if part  == Parts::ONE || part == Parts::BOTH {
            let (sol2, solv2_time) = evaluate_function(input, fn2);
            println!("    Part2 in {}\t=> {}",  solv2_time.pretty(), sol2)
        }
    }

    fn run_day_with_gen<A, G, T, F1, F2>(&self, input: &str, gen: G, fn1: F1, fn2: F2)
    where T: Display,
    G: Fn(&str) -> A,
    A: Clone,
    F1: Fn(&A) -> T,
    F2: Fn(&A) -> T
    {
        let part = self.part();
        println!("Day {}", self.day());
        let (input, gen_time) = evaluate_function(input, gen);
        println!("    Generator in {}", gen_time.pretty());
        if part  == Parts::ONE || part == Parts::BOTH {
            let (sol1, solv1_time) = evaluate_function(&input, fn1);
            println!("    Part1 in {}\t=> {}",  solv1_time.pretty(), sol1)
        }
        if part  == Parts::TWO || part == Parts::BOTH {
            let (sol2, solv2_time) = evaluate_function(&input, fn2);
            println!("    Part2 in {}\t=> {}",  solv2_time.pretty(), sol2)
        }
    }
}

fn evaluate_function<A, T, F: FnMut(A) -> T>(input: A, mut func: F) -> (T, Duration) {
    let now = Instant::now();
    let result = func(input);
    (result, Instant::now()-now)
}

trait Pretty {
    fn pretty(&self) -> String;
}

impl Pretty for Duration {
    fn pretty(&self) -> String {
        if self.as_secs() != 0 {
            format!("{} s", self.as_secs())
        } else if self.as_millis() != 0 {
            format!("{} ms", self.as_millis())
        } else if self.as_micros() != 0 {
            format!("{} us", self.as_micros())
        } else {
            format!("{} ns", self.as_nanos())
        }
    }
}

#[derive(Clone, Copy,PartialEq)]
pub enum Parts {
    ONE,
    TWO,
    BOTH,
    NONE
}

