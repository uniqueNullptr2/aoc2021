use std::{str::FromStr, collections::HashSet};

fn transformations(x: i32, y: i32, z: i32) -> [(i32,i32,i32); 24] {
    [
        (x,y,z),
        (x,-z,y),
        (x,-y,-z),
        (x,z,-y),

        (y,-x,z),
        (y,-z,-x),
        (y,x,-z),
        (y,z,x),

        (z,y,-x),
        (z,x,y),
        (z,-y,x),
        (z,-x,-y),

        (-x,-y,z),
        (-x,-z,-y),
        (-x,y,-z),
        (-x,z,y),

        (-y,x,z),
        (-y,-z,x),
        (-y,-x,-z),
        (-y,z,-x),

        (-z,y,x),
        (-z,-x,y),
        (-z,-y,-x),
        (-z,x,-y)
    ]
}

pub fn input_generator(input: &str) -> Vec<Vec<(i32,i32,i32)>> {
    let mut split = input.trim().split("--- ");
    split.next().unwrap();
    let mut v = vec!();
    while let Some(s) = split.next() {
        let mut lines = s.trim().lines();
        lines.next().unwrap();
        let mut vv = vec!();
        while let Some(l) = lines.next() {
            let mut tup = l.split(',').map(|f| i32::from_str(f).unwrap());
            vv.push((tup.next().unwrap(),tup.next().unwrap(),tup.next().unwrap()))
        }
        v.push(vv);
    }

    v
}


fn unify_probes(vec: &mut Vec<(i32,i32,i32)>, other: &Vec<(i32,i32,i32)>) -> bool {
    let  probes: Vec<[(i32,i32,i32);24]> = other.iter().map(|(a,b,c)| transformations(*a, *b, *c)).collect();
    let mut set = HashSet::new();
    for p in vec.iter() {
        set.insert(*p);
    }

    let mut e = 0;
    let  flag = false;
    for i in 0..24 {
        let mut c = 0;
        for ps in &probes{
            if set.contains(&ps[i]) {
                c += 1;
            }
        }
        if c > 0 {
            println!("found {} matches", c);
        }
        if c >= 12 {
            e = i;
            break;
        }
    }

    if flag {
        println!("appended yo");
        for ps in probes {
            if !set.contains(&ps[e]) {
                vec.push(ps[e]);
            }
        }
        true
    } else {
        false
    }
}

pub fn solve_part1(input: &[Vec<(i32,i32,i32)>]) -> usize {
    let mut probes = input[0].clone();
    let mut v: Vec<Vec<(i32,i32,i32)>> = input[1..].iter().cloned().collect();
    loop {
        v = v.into_iter().filter(|vv| !unify_probes(&mut probes, vv)).collect();
        if v.len() == 0 {
            break;
        }
    }
    probes.len()
}



pub fn solve_part2(_input: &[Vec<(i32,i32,i32)>]) -> i32 {
    todo!()
}