pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

pub fn solve_part1(vec: &[Vec<u32>]) -> u32 {
    find_low_points(vec).iter().map(|(i,e)| vec[*i][*e]+1).sum()
}


pub fn solve_part2(vec: &[Vec<u32>]) -> u32 {
    let mut set = vec!(vec!(false; vec[0].len()); vec.len());
    let mut v = find_low_points(vec).into_iter().map(|(i,e)| {
        count_basin(i, e, vec, &mut set)
    }).collect::<Vec<usize>>();
    v.sort_by(|a, b| b.cmp(a));

    (v[0]*v[1]*v[2]) as u32
}

fn count_basin(i: usize, e: usize, vec: &[Vec<u32>], set: &mut Vec<Vec<bool>>) -> usize{
    let mut stack = vec!((i, e));
    let mut c = 0;
    while let Some((i, e)) = stack.pop() {
        if !set[i][e] {
            set[i][e] = true;
            c += 1;
            for p in get_surrounding(i,e, vec).iter().filter_map(|o|o.filter(|(i,e,u)| *u < 9 && !set[*i][*e])) {
                stack.push((p.0, p.1));
            }
        }
    }
    c
}

fn find_low_points(vec: &[Vec<u32>]) -> Vec<(usize, usize)> {
    vec.iter()
        .enumerate()
        .map(|(i, v)| v.iter()
            .enumerate()
            .map(move |(e, n)| (i, e, n)))
        .flatten()
        .filter(|(i, e, n)| {
            let t = get_surrounding(*i, *e, vec)
                .iter()
                .filter(|f| if let Some((_,_, u)) = f{ u > n} else {true}).count();
            t == 4
        })
        .map(|(i, e, _)| (i, e))
        .collect()
}


//TODO insert rust magic
fn get_surrounding(i: usize, e: usize, vec: &[Vec<u32>]) -> [Option<(usize, usize,u32)>;4] {
    [i.checked_sub(1).map(|i| vec.get(i).map(|v| v.get(e).map(|u| (i, e, *u)))).flatten().flatten()
    ,vec.get(i+1).map(|v| v.get(e).map(|u| (i+1, e, *u))).flatten()
    ,vec.get(i).map(|v| e.checked_sub(1).map(|e| v.get(e).map(|u| (i, e, *u)))).flatten().flatten()
    ,vec.get(i).map(|v| v.get(e+1).map(|u| (i, e+1, *u))).flatten()]
}
