use itertools::Itertools;

pub fn input_generator(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines().map(|s| {
        let mut split = s.split(" | ");
        let code: Vec<String> = split.next().unwrap().split_whitespace().map(|s| s.to_owned()).collect();
        let num = split.next().unwrap().split_whitespace().map(|s| s.to_owned()).collect();
        (code, num)
    })
    .collect()
}

pub fn solve_part1(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input.iter().map(|(_,n)| n.iter().filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7).count()).sum()
}


pub fn solve_part2(input: &[(Vec<String>, Vec<String>)]) -> usize {
    input.iter()
        .map(|(c,n)| {
            let map = Mapping::parse(c);
            map.number(n)
        }).sum()
}


struct Mapping {
    map: [char; 7]
}

impl Mapping {
    pub fn parse(input: &[String])  -> Self{
        let mut tmp = [0; 7];
        let mut one = None;
        let mut four = None;
        for s in input {
            for c in s.chars() {
                tmp[c as usize - 97] += 1;
            }
            if s.len() == 2 {
                one = Some(s);
            } else if s.len() == 4 {
                four = Some(s);
            }
        }
        let mut dec = [' '; 7];
        for (i,n) in tmp.iter().enumerate() {
            let c = match n {
                6 => 'b',
                4 => 'e',
                9 => 'f',
                8 if one.unwrap().contains((i+97) as u8 as char) => 'c',
                8 => 'a',
                7 if four.unwrap().contains((i+97) as u8 as char) => 'd',
                7 => 'g',
                _ => ' '
            };
            dec[i] = c;
        }
        Self{map: dec}
    }

    fn number(&self, s: &[String]) -> usize {
        let v: Vec<usize> = s.iter().map(|s| {
            let s: String = s.chars().map(|c| self.map[c as usize -97]).sorted().collect();
            match s.as_str() {
                "abcefg" => 0,
                "cf" => 1,
                "acdeg" => 2,
                "acdfg" => 3,
                "bcdf" => 4,
                "abdfg" => 5,
                "abdefg" => 6,
                "acf" => 7,
                "abcdefg" => 8,
                "abcdfg" => 9,
                _ => 1337
            }
        }).collect();
        v[0] * 1000 + v[1] * 100 + v[2] * 10 + v[3]
    }
}
/*
a -> 8
b -> 6
c -> 8
d -> 7
e -> 4
f -> 9
g -> 7

8 + in 1 -> c
8 -> a

7 + in 4 -> d
7 -> g
*/