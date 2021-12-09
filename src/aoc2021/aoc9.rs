use std::collections::HashSet;

pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect()
}

pub fn solve_part1(vec: &[Vec<u32>]) -> u32 {
    find_low_points(vec).iter().map(|(i,e)| vec[*i][*e]+1).sum()
}


pub fn solve_part2(vec: &[Vec<u32>]) -> u32 {
    
    let mut v = find_low_points(vec).into_iter().map(|(i,e)| {
        let mut set = HashSet::new();
        count_basin(i, e, vec, &mut set)
    }).collect::<Vec<usize>>();
    v.sort_by(|a, b| b.cmp(a));

    (v[0]*v[1]*v[2]) as u32
}

fn count_basin(i: usize, e: usize, vec: &[Vec<u32>], set: &mut HashSet<(usize,usize)>) -> usize{
    if set.insert((i,e)) {
        for p in get_surrounding2(i,e, vec) {
            count_basin(p.0, p.1, vec, set);
        }
    }
    set.len()
}
fn find_low_points(vec: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let l = vec.len()-1;
    let m = vec[0].len()-1;
    let mut v = vec!();
    for i in 0..=l {
        for e in 0..=m {
            let n = vec[i][e];
            let t = get_surrounding(i, e, vec)
                    .iter()
                    .filter(|f| if let Some(u) = f{ u > &n} else {true}).count();
            if t == 4 {
                v.push((i, e));
            }
        }
    }
    v
}

fn get_surrounding2(i: usize, e: usize, vec: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut r = vec!();
    let l = vec.len()-1;
    let m = vec[0].len()-1;
    if i>0 && vec[i-1][e] < 9{
        r.push( (i-1,e));
    }
    if i<l && vec[i+1][e] < 9{
        r.push((i+1,e));
    }
    if e>0 && vec[i][e-1] < 9{
        r.push((i,e-1));
    }
    if e<m && vec[i][e+1] < 9{
        r.push((i,e+1));
    }
    r
}

fn get_surrounding(i: usize, e: usize, vec: &[Vec<u32>]) -> [Option<u32>;4] {
    let mut r = [None; 4];
    let l = vec.len()-1;
    let m = vec[0].len()-1;
    if i>0{
        r[0] = Some(vec[i-1][e]);
    } else {
        r[0] = None;
    }
    if i<l{
        r[1] = Some(vec[i+1][e]);
    } else {
        r[1] = None;
    }
    if e>0{
        r[2] = Some(vec[i][e-1]);
    } else {
        r[2] = None;
    }
    if e<m{
        r[3] = Some(vec[i][e+1]);
    } else {
        r[3] = None;
    }
    r
}