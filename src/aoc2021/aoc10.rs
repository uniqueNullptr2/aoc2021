use std::collections::VecDeque;

pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

pub fn solve_part1(_vec: &[String]) -> usize {
    let it = _vec.iter()
        .map(|s| {
            let mut stack = VecDeque::new();
            let mut cc = '_';
            for c in s.chars() {
                match stack.back() {
                    Some(_) if (c == '(' || c =='[' || c == '{' || c == '<') => stack.push_back(c),
                    Some('(') if c == ')' => {stack.pop_back();},
                    Some('(')  => {cc = c;break;},
                    Some('[') if c == ']' => {stack.pop_back();},
                    Some('[')  => {cc = c;break;},
                    Some('{') if c == '}' => {stack.pop_back();},
                    Some('{')  => {cc = c;break;},
                    Some('<') if c == '>' => {stack.pop_back();},
                    Some('<')  => {cc = c;break;},
                    None => stack.push_back(c),
                    _ => ()
                }
            }
            match cc {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 1337
            }
        });
    let mut c0 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;
    for p in it {
        match p {
            3 => c0 += 1,
            57 => c1 += 1,
            1197 => c2 += 1,
            25137 => c3 += 1,
            _ => ()
        }
    }
    3*c0 + 57*c1 + 1197*c2 + 25137* c3
}


pub fn solve_part2(_vec: &[String]) -> usize {
    let mut scores: Vec<usize> = _vec.iter()
        .filter_map(|s| {
            let mut stack = VecDeque::new();
            let mut broken = false;
            for c in s.chars() {
                match stack.back() {
                    Some(_) if (c == '(' || c =='[' || c == '{' || c == '<') => stack.push_back(c),
                    Some('(') if c == ')' => {stack.pop_back();},
                    Some('(')  => {broken=true;break;},
                    Some('[') if c == ']' => {stack.pop_back();},
                    Some('[')  => {broken=true;break;},
                    Some('{') if c == '}' => {stack.pop_back();},
                    Some('{')  => {broken=true;break;},
                    Some('<') if c == '>' => {stack.pop_back();},
                    Some('<')  => {broken=true;break;},
                    None => stack.push_back(c),
                    _ => ()
                }
            }
            if broken {
                None
            } else {
                Some(stack)
            }
        }).map(|mut stack| {
            let mut score = 0;
            while let Some(c) = stack.pop_back() {
                score *= 5;
                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                };
            }
            score
        }).collect();
        scores.sort_unstable();
        scores[(scores.len()-1)/2]
}