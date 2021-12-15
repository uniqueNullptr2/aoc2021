use std::collections::{BinaryHeap, HashSet};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Node {
    value: u32,
    index: usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value).then(self.index.cmp(&other.index))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match other.value.partial_cmp(&self.value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.index.partial_cmp(&other.index)
    }
}

pub fn input_generator(input: &str) -> (usize, usize,Vec<Node>, Vec<Node>) {
    let c = input.lines().count();

    let v: Vec<Node> = input
    .lines()
    .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u32))
    .flatten()
    .enumerate()
    .map(|(i,n)| Node{value: n, index: i}).collect();
    let mut vv = vec!();
    let w = v.len() / c;
    for i in 0..c {
        for e in 0..5 {
            for x in 0..w {
                let mut n = v[i*w+x].value + e as u32;
                if n > 9 {
                    n -= 9;
                }
                vv.push(Node{value: n, index: i*w*5+e*w+x})
            }
        }
    }

    for i in 1..5 {
        for e in 0..c {
            for f in 0..w*5 {
                let mut n = vv[e*w*5+f].value + i as u32;
                if n > 9 {
                    n -= 9;
                }
                vv.push(Node{value:n, index: 5*i*e*w*5+f});
            }
        }
    }
    (v.len()/c, c, v, vv)
}


pub fn solve_part1(input: &(usize, usize,Vec<Node>, Vec<Node>)) -> u32 {
    let (w,h,v, _) = input;
    dijkstra(v, *w, *h)
}



pub fn solve_part2(input: &(usize, usize,Vec<Node>, Vec<Node>)) -> u32 {
    let (mut w,mut h, _, v) = input;
    w *= 5;
    h *= 5;
    dijkstra(v, w, h)
}

fn dijkstra(v: &[Node], w: usize, h: usize) -> u32 {
    let mut result = 0;
    let mut set = HashSet::new();
    let mut q = BinaryHeap::new();
    q.push(v[0]);
    while let Some(node) = q.pop() {
        if node.index == v.len() -1 {
            result = node.value;
            break;
        } else if !set.contains(&node.index) {
            set.insert(node.index);
            for i in get_points(w, h, node.index, &set).into_iter().flatten() {
                q.push(Node{value:node.value+v[i].value, index:i});
            }
        }
    }
    result - v[0].value
}

fn get_points(width: usize, height: usize, index: usize, set: &HashSet<usize>) -> [Option<usize>;4]{
    let x = index % width;
    let y = (index - x) / width;
    [
        x.checked_sub(1).map(|_| index-1).filter(|n| !set.contains(n)),
        Some(x+1).filter(|n| *n < width).map(|_| index+1).filter(|n| !set.contains(n)),
        y.checked_sub(1).map(|_| index-width).filter(|n| !set.contains(n)),
        Some(y+1).filter(|n| *n < height).map(|_| index+width).filter(|n| !set.contains(n))
    ]
}