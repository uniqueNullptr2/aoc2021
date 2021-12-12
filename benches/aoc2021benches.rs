use criterion::{Criterion, criterion_group};
use aoc2021::aoc2021::{aoc1, aoc2, aoc3, aoc4, aoc5, aoc6, aoc7, aoc9, aoc8, aoc10, aoc11, aoc12};



pub fn day1_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day1.txt").unwrap();
    c.bench_function("aoc2021_day1_gen",|b| b.iter(|| aoc1::generate_1(&s)));
}
pub fn day1_solve1(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day1.txt").unwrap();
    let f = aoc1::generate_1(&s);
    c.bench_function("aoc2021_day1_solve1",|b| b.iter(|| aoc1::solve_part1(&f)));
}
pub fn day1_solve2(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day1.txt").unwrap();
    let f = aoc1::generate_1(&s);
    c.bench_function("aoc2021_day1_solve1",|b| b.iter(|| aoc1::solve_part2(&f)));
}


pub fn day2_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day2.txt").unwrap();
    c.bench_function("aoc2021_day2_gen",|b| b.iter(|| aoc2::input_generator(&s)));
}
pub fn day2_solve1(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day2.txt").unwrap();
    let f = aoc2::input_generator(&s);
    c.bench_function("aoc2021_day2_solve1",|b| b.iter(|| aoc2::solve_part1(&f)));
}
pub fn day2_solve2(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day2.txt").unwrap();
    let f = aoc2::input_generator(&s);
    c.bench_function("aoc2021_day2_solve1",|b| b.iter(|| aoc2::solve_part2(&f)));
}


pub fn day3_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day3.txt").unwrap();
    c.bench_function("aoc2021_day3_gen",|b| b.iter(|| aoc3::input_generator(&s)));
}
pub fn day3_solve1(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day3.txt").unwrap();
    let f = aoc3::input_generator(&s);
    c.bench_function("aoc2021_day3_solve1",|b| b.iter(|| aoc3::solve_part1(&f)));
}
pub fn day3_solve2(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day3.txt").unwrap();
    let f = aoc3::input_generator(&s);
    c.bench_function("aoc2021_day3_solve1",|b| b.iter(|| aoc3::solve_part2(&f)));
}


pub fn day4_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day4.txt").unwrap();
    c.bench_function("aoc2021_day4_gen",|b| b.iter(|| aoc4::input_generator(&s)));
}
pub fn day4_solve1(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day4.txt").unwrap();
    let f = aoc4::input_generator(&s);
    c.bench_function("aoc2021_day4_solve1",|b| b.iter(|| aoc4::solve_part1(&f)));
}
pub fn day4_solve2(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day4.txt").unwrap();
    let f = aoc4::input_generator(&s);
    c.bench_function("aoc2021_day4_solve1",|b| b.iter(|| aoc4::solve_part2(&f)));
}


pub fn day5_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
    c.bench_function("aoc2021_day5_gen",|b| b.iter(|| aoc5::input_generator(&s)));
}
pub fn day5_solve1(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
    let f = aoc5::input_generator(&s);
    c.bench_function("aoc2021_day5_solve1",|b| b.iter(|| aoc5::solve_part1(&f)));
}
pub fn day5_solve2_arr(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
    let f = aoc5::input_generator(&s);
    c.bench_function("aoc2021_day5_solve1",|b| b.iter(|| aoc5::solve_part2_arr(&f)));
}
pub fn day5_solve2_map(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
    let f = aoc5::input_generator(&s);
    c.bench_function("aoc2021_day5_solve1",|b| b.iter(|| aoc5::solve_part2_map(&f)));
}
pub fn day5_solve2_vec(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
    let f = aoc5::input_generator(&s);
    c.bench_function("aoc2021_day5_solve1",|b| b.iter(|| aoc5::solve_part2_vec(&f)));
}
pub fn day5_solve2_btree(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day5.txt").unwrap();
    let f = aoc5::input_generator(&s);
    c.bench_function("aoc2021_day5_solve1",|b| b.iter(|| aoc5::solve_part2_btree(&f)));
}

pub fn day6_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day6.txt").unwrap();
    c.bench_function("aoc2021_day6_gen",|b| b.iter(|| aoc6::input_generator(&s)));
}
pub fn day6_solve1(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day6.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc6::input_generator(&s2);
    c.bench_function("aoc2021_day6_solve1",|b| b.iter_batched(|| v.clone(), |mut v| aoc6::solve_part1(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day6_solve2(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day6.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc6::input_generator(&s2);
    c.bench_function("aoc2021_day6_solve2",|b| b.iter_batched(|| v.clone(), |mut v| aoc6::solve_part2(&mut v), criterion::BatchSize::SmallInput));
}


pub fn day7_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day7.txt").unwrap();
    c.bench_function("aoc2021_day7_gen",|b| b.iter(|| aoc7::input_generator(&s)));
}
pub fn day7_solve1(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day7.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc7::input_generator(&s2);
    c.bench_function("aoc2021_day7_solve1",|b| b.iter_batched(|| v.clone(), |mut v| aoc7::solve_part1(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day7_solve2(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day7.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc7::input_generator(&s2);
    c.bench_function("aoc2021_day7_solve2",|b| b.iter_batched(|| v.clone(), |mut v| aoc7::solve_part2(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day8_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day8.txt").unwrap();
    c.bench_function("aoc2021_day8_gen",|b| b.iter(|| aoc8::input_generator(&s)));
}
pub fn day8_solve1(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day8.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc8::input_generator(&s2);
    c.bench_function("aoc2021_day8_solve1",|b| b.iter_batched(|| v.clone(), |mut v| aoc8::solve_part1(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day8_solve2(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day8.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc8::input_generator(&s2);
    c.bench_function("aoc2021_day8_solve2",|b| b.iter_batched(|| v.clone(), |mut v| aoc8::solve_part2(&mut v), criterion::BatchSize::SmallInput));
}

pub fn day9_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day9.txt").unwrap();
    c.bench_function("aoc2021_day9_gen",|b| b.iter(|| aoc9::input_generator(&s)));
}
pub fn day9_solve1(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day9.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc9::input_generator(&s2);
    c.bench_function("aoc2021_day9_solve1",|b| b.iter_batched(|| v.clone(), |mut v| aoc9::solve_part1(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day9_solve2(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day9.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc9::input_generator(&s2);
    c.bench_function("aoc2021_day9_solve2",|b| b.iter_batched(|| v.clone(), |mut v| aoc9::solve_part2(&mut v), criterion::BatchSize::SmallInput));
}

pub fn day10_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day10.txt").unwrap();
    c.bench_function("aoc2021_day10_gen",|b| b.iter(|| aoc10::input_generator(&s)));
}
pub fn day10_solve1(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day10.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc10::input_generator(&s2);
    c.bench_function("aoc2021_day10_solve1",|b| b.iter_batched(|| v.clone(), |mut v| aoc10::solve_part1(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day10_solve2(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day10.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc10::input_generator(&s2);
    c.bench_function("aoc2021_day10_solve2",|b| b.iter_batched(|| v.clone(), |mut v| aoc10::solve_part2(&mut v), criterion::BatchSize::SmallInput));
}

pub fn day11_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day11.txt").unwrap();
    c.bench_function("aoc2021_day11_gen",|b| b.iter(|| aoc11::input_generator(&s)));
}
pub fn day11_solve1(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day11.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc11::input_generator(&s2);
    c.bench_function("aoc2021_day11_solve1",|b| b.iter_batched(|| v.clone(), |mut v| aoc11::solve_part1(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day11_solve2(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day11.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc11::input_generator(&s2);
    c.bench_function("aoc2021_day11_solve2",|b| b.iter_batched(|| v.clone(), |mut v| aoc11::solve_part2(&mut v), criterion::BatchSize::SmallInput));
}

pub fn day12_gen(c: &mut Criterion) {
    let s = std::fs::read_to_string("input/2021/day12.txt").unwrap();
    c.bench_function("aoc2021_day12_gen",|b| b.iter(|| aoc12::input_generator(&s)));
}
pub fn day12_solve1(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day12.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc12::input_generator(&s2);
    c.bench_function("aoc2021_day12_solve1",|b| b.iter_batched(|| v.clone(), |mut v| aoc12::solve_part1(&mut v), criterion::BatchSize::SmallInput));
}
pub fn day12_solve2(c: &mut Criterion) {
    let s1 = std::fs::read_to_string("input/2021/day12.txt").unwrap();
    let s2 = s1.trim_end_matches("\n");
    let v = aoc12::input_generator(&s2);
    c.bench_function("aoc2021_day12_solve2",|b| b.iter_batched(|| v.clone(), |mut v| aoc12::solve_part2(&mut v), criterion::BatchSize::SmallInput));
}

criterion_group!( aoc2021,  day1_gen, day1_solve1, day1_solve2,
                            day2_gen, day2_solve1, day2_solve2,
                            day2_gen, day2_solve1, day2_solve2,
                            day3_gen, day3_solve1, day3_solve2,
                            day4_gen, day4_solve1, day4_solve2,
                            day5_gen, day5_solve1, day5_solve2_arr, day5_solve2_map, day5_solve2_btree, day5_solve2_vec,
                            day6_gen, day6_solve1, day6_solve2,
                            day7_gen, day7_solve1, day7_solve2,
                            day8_gen, day8_solve1, day8_solve2,
                            day9_gen, day9_solve1, day9_solve2,
                            day10_gen, day10_solve1, day10_solve2,
                            day11_gen, day11_solve1, day11_solve2,
                            day12_gen, day12_solve1, day12_solve2);