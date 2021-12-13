use criterion::{Criterion, criterion_group};
use aoc2021::aoc2021::{aoc1, aoc2, aoc3, aoc4, aoc5, aoc6, aoc7, aoc9, aoc8, aoc10, aoc11, aoc12, aoc13};


pub fn day1(c: &mut Criterion) {
    c.bench_function("aoc2021_day01_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day1.txt").unwrap()
        }, |s| aoc1::generate_1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day01_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day1.txt").unwrap();
            aoc1::generate_1(&s)
        }, |s| aoc1::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day01_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day1.txt").unwrap();
            aoc1::generate_1(&s)
        }, |s| aoc1::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}

pub fn day2(c: &mut Criterion) {
    c.bench_function("aoc2021_day02_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day2.txt").unwrap()
        }, |s| aoc2::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day02_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day2.txt").unwrap();
            aoc2::input_generator(&s)
        }, |s| aoc2::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day02_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day2.txt").unwrap();
            aoc2::input_generator(&s)
        }, |s| aoc2::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}

pub fn day3(c: &mut Criterion) {
    c.bench_function("aoc2021_day03_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day3.txt").unwrap()
        }, |s| aoc3::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day03_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day3.txt").unwrap();
            aoc3::input_generator(&s)
        }, |s| aoc3::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day03_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day3.txt").unwrap();
            aoc3::input_generator(&s)
        }, |s| aoc3::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}


pub fn day4(c: &mut Criterion) {
    c.bench_function("aoc2021_day04_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day4.txt").unwrap()
        }, |s| aoc4::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day04_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day4.txt").unwrap();
            aoc4::input_generator(&s)
        }, |s| aoc4::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day04_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day4.txt").unwrap();
            aoc4::input_generator(&s)
        }, |s| aoc4::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}


pub fn day5(c: &mut Criterion) {
    c.bench_function("aoc2021_day05_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day5.txt").unwrap()
        }, |s| aoc5::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day05_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
            aoc5::input_generator(&s)
        }, |s| aoc5::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day05_solve2_arr",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
            aoc5::input_generator(&s)
        }, |s| aoc5::solve_part2_arr(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day05_solve2_vec",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
            aoc5::input_generator(&s)
        }, |s| aoc5::solve_part2_vec(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day05_solve2_map",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
            aoc5::input_generator(&s)
        }, |s| aoc5::solve_part2_map(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day05_solve2_btree",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
            aoc5::input_generator(&s)
        }, |s| aoc5::solve_part2_btree(&s), criterion::BatchSize::SmallInput)
    });
}

pub fn day6(c: &mut Criterion) {
    c.bench_function("aoc2021_day06_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day6.txt").unwrap()
        }, |s| aoc6::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day06_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day6.txt").unwrap();
            aoc6::input_generator(&s)
        }, |mut s| aoc6::solve_part1(&mut s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day06_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day6.txt").unwrap();
            aoc6::input_generator(&s)
        }, |mut s| aoc6::solve_part2(&mut s), criterion::BatchSize::SmallInput)
    });
}

pub fn day7(c: &mut Criterion) {
    c.bench_function("aoc2021_day07_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day7.txt").unwrap()
        }, |s| aoc7::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day07_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day7.txt").unwrap();
            aoc7::input_generator(&s)
        }, |s| aoc7::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day07_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day7.txt").unwrap();
            aoc7::input_generator(&s)
        }, |s| aoc7::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}

pub fn day8(c: &mut Criterion) {
    c.bench_function("aoc2021_day08_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day8.txt").unwrap()
        }, |s| aoc8::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day08_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day8.txt").unwrap();
            aoc8::input_generator(&s)
        }, |s| aoc8::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day08_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day8.txt").unwrap();
            aoc8::input_generator(&s)
        }, |s| aoc8::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}


pub fn day9(c: &mut Criterion) {
    c.bench_function("aoc2021_day09_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day9.txt").unwrap()
        }, |s| aoc9::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day09_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day9.txt").unwrap();
            aoc9::input_generator(&s)
        }, |s| aoc9::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day09_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day9.txt").unwrap();
            aoc9::input_generator(&s)
        }, |s| aoc9::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}

pub fn day10(c: &mut Criterion) {
    c.bench_function("aoc2021_day10_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day10.txt").unwrap()
        }, |s| aoc10::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day10_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day10.txt").unwrap();
            aoc10::input_generator(&s)
        }, |s| aoc10::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day10_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day10.txt").unwrap();
            aoc10::input_generator(&s)
        }, |s| aoc10::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}


pub fn day11(c: &mut Criterion) {
    c.bench_function("aoc2021_day11_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day11.txt").unwrap()
        }, |s| aoc11::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day11_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day11.txt").unwrap();
            aoc11::input_generator(&s)
        }, |mut s| aoc11::solve_part1(&mut s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day11_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day11.txt").unwrap();
            aoc11::input_generator(&s)
        }, |mut s| aoc11::solve_part2(&mut s), criterion::BatchSize::SmallInput)
    });
}

pub fn day12(c: &mut Criterion) {
    c.bench_function("aoc2021_day12_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day12.txt").unwrap()
        }, |s| aoc12::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day12_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day12.txt").unwrap();
            aoc12::input_generator(&s)
        }, |s| aoc12::solve_part1(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day12_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day12.txt").unwrap();
            aoc12::input_generator(&s)
        }, |s| aoc12::solve_part2(&s), criterion::BatchSize::SmallInput)
    });
}
pub fn day13(c: &mut Criterion) {
    c.bench_function("aoc2021_day13_gen",|b| {
        b.iter_batched(|| {
            std::fs::read_to_string("input/2021/day13.txt").unwrap()
        }, |s| aoc13::input_generator(&s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day13_solve1",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day13.txt").unwrap();
            aoc13::input_generator(&s)
        }, |mut s| aoc13::solve_part1(&mut s), criterion::BatchSize::SmallInput)
    });

    c.bench_function("aoc2021_day13_solve2",|b| {
        b.iter_batched(|| {
            let s = std::fs::read_to_string("input/2021/day13.txt").unwrap();
            aoc13::input_generator(&s)
        }, |mut s| aoc13::solve_part2(&mut s), criterion::BatchSize::SmallInput)
    });
}

criterion_group!( aoc2021,  day1,
                            day2,
                            day3,
                            day4,
                            day5,
                            day6,
                            day7,
                            day8,
                            day9,
                            day10,
                            day11,
                            day12,
                            day13);