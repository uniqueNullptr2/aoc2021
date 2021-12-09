pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(str::parse).collect::<Result<Vec<usize>,_>>().unwrap()
}

pub fn solve_part1(_vec: &[usize]) -> u32 {
    0
}


pub fn solve_part2(_vec: &[usize]) -> u32 {
    0
}