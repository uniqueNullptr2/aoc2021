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
    stack.push_back((graph.get_start(), mask));
    let mut c = 0;
    while let Some((last, mask)) = stack.pop_back() {
        let (_, _, _, _, indices) = graph.get(last);
        for i in indices {
            let (_, start, end, lower,_) = graph.get(*i);
            if *end {
                c += 1;
            } else if *start {

            } else {
                let mut mask = mask;
                if *lower && !mask.get(*i) {
                    mask.set(*i, true);
                    stack.push_back((*i, mask))
                } else if !*lower{
                    stack.push_back((*i, mask))
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
    let mut c = 0;
    while let Some((last, mask, double)) = stack.pop_back() {
        let (_, _, _, _, indices) = graph.get(last);
        for i in indices {
            let (_, start, end, lower,_) = graph.get(*i);
            if *end {
                c += 1;
            } else if *start {

            } else {
                let mut mask = mask;
                if *lower && !mask.get(*i) {
                    mask.set(*i, true);
                    stack.push_back((*i, mask, double))
                } else if *lower && !double{
                    stack.push_back((*i, mask, true))
                } else if !*lower{
                    stack.push_back((*i, mask, double))
                }
            }
        }
    }
    c
}