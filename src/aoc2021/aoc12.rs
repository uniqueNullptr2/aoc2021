use std::{collections::{VecDeque, HashMap}, fmt::Display};

use bitmaps::Bitmap;

#[derive(Clone, Debug)]
pub struct Graph {
    g: Vec<(String,bool, bool, bool, Vec<usize>)>
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let g = self.g
            .iter()
            .map(|(s, ss, e, l,ii)| {
                format!("{} => [Start: {}, End: {} Lower: {}, Connected: {:?}]", s, ss, e, l, ii.iter().map(|i| self.g[*i].0.to_owned()).collect::<Vec<String>>())
            }).collect::<Vec<String>>();
        write!(f, "{}", g.join("\n"))
    }
}
impl Graph {
    pub fn parse(input: &str) -> Self {
        let mut map = HashMap::new();
        for (a, b) in input.lines()
            .map(|l| {
                let mut split = l.trim().split('-');
                (split.next().unwrap(), split.next().unwrap())
            }) {
                map.entry(a.to_owned()).or_insert(vec!()).push(b.to_owned());
                map.entry(b.to_owned()).or_insert(vec!()).push(a.to_owned());
            }
        let tmp = map.iter().map(|(s,_)| s.to_owned()).collect::<Vec<String>>();

        let g = tmp.iter()
            .map(|k| (k.to_owned(), k=="start", k=="end", &k.to_lowercase() == k, map.get(k).unwrap().iter().map(|s| tmp.iter().position(|r| r==s ).unwrap()).collect())).collect();
        Self{g}
        // let v = map.into_iter().enumerate()
    }

    pub fn get_start(&self) -> usize {
        self.g.iter().position(|(_, s,_,_,_)| *s).unwrap()
    }
    pub fn get(&self, i: usize) -> &(String,bool,bool,bool,Vec<usize>)  {
        &self.g[i]
    }
}
pub fn input_generator(input: &str) -> Graph {
    Graph::parse(input)
}

pub fn solve_part1(graph: &Graph) -> usize {
    let mut stack = VecDeque::new();
    let mask = Bitmap::<64>::new();
    stack.push_back((graph.get_start(), mask, false));
    let i = graph.get_start();
    let mut map = HashMap::new();
    solve_part1_rec(graph, i, &mut map, mask)
}
pub fn solve_part1_rec(graph: &Graph, last: usize, map: &mut HashMap<(usize, Bitmap<64>), usize>, mask: Bitmap<64>) -> usize {

    let (_, _, _, _, indices) = graph.get(last);
    let mut c: usize = 0;
    for i in indices {
        let (_, start, end, lower,_) = graph.get(*i);
        if *end {
            c = c.checked_add(1).unwrap();
        } else if *start {
        } else {
            let mut mask = mask;
            if *lower && !mask.get(*i) {
                mask.set(*i, true);
                if map.contains_key(&(*i, mask)) {
                    c = c.checked_add(*map.get(&(*i, mask)).unwrap()).unwrap();
                } else {
                    let t = solve_part1_rec(graph, *i, map, mask);
                    c = c.checked_add(t).unwrap();
                    map.insert((*i, mask), t);
                }
            }else if !*lower{
                if map.contains_key(&(*i, mask)) {
                    c = c.checked_add(*map.get(&(*i, mask)).unwrap()).unwrap();
                } else {
                    let t = solve_part1_rec(graph, *i, map, mask);
                    c = c.checked_add(t).unwrap();
                    map.insert((*i, mask), t);
                }
            }
        }
    }
c
}

pub fn solve_part2(graph: &Graph) -> usize {
    let mut stack = VecDeque::new();
    let mask = Bitmap::<64>::new();
    stack.push_back((graph.get_start(), mask, false));
    let i = graph.get_start();
    let mut map = HashMap::new();
    solve_part2_rec(graph, i, &mut map, mask, false)
}
pub fn solve_part2_rec(graph: &Graph, last: usize, map: &mut HashMap<(usize, Bitmap<64>, bool), usize>, mask: Bitmap<64>, double: bool) -> usize {

        let (_, _, _, _, indices) = graph.get(last);
        let mut c: usize = 0;
        for i in indices {
            let (_, start, end, lower,_) = graph.get(*i);
            if *end {
                c = c.checked_add(1).unwrap();
            } else if *start {
            } else {
                let mut mask = mask;
                if *lower && !mask.get(*i) {
                    mask.set(*i, true);
                    if map.contains_key(&(*i, mask, double)) {
                        c = c.checked_add(*map.get(&(*i, mask, double)).unwrap()).unwrap();
                    } else {
                        let t = solve_part2_rec(graph, *i, map, mask, double);
                        c = c.checked_add(t).unwrap();
                        map.insert((*i, mask, double), t);
                    }
                } else if *lower && !double{
                    if map.contains_key(&(*i, mask, true)) {
                        c = c.checked_add(*map.get(&(*i, mask, true)).unwrap()).unwrap();
                    } else {
                        let t = solve_part2_rec(graph, *i, map, mask, true);
                        c = c.checked_add(t).unwrap();
                        map.insert((*i, mask, true), t);
                    }
                } else if !*lower{
                    if map.contains_key(&(*i, mask, double)) {
                        c = c.checked_add(*map.get(&(*i, mask, double)).unwrap()).unwrap();
                    } else {
                        let t = solve_part2_rec(graph, *i, map, mask, double);
                        c = c.checked_add(t).unwrap();
                        map.insert((*i, mask, double), t);
                    }
                }
            }
        }
    c
}