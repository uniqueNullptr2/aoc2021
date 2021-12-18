use std::str::FromStr;

#[derive(Clone,Copy)]
pub struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32
}

impl Target {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self{x1,x2,y1,y2}
    }
    pub fn hit(&self, dx: i32, dy: i32) -> bool {
        let mut ddx = dx;
        let mut ddy = dy;
        let mut x = 0;
        let mut y = 0;
        loop {
            if x > self.x2 {
                return false;
            } else if y < self.y2 {
                return false
            } else if x >= self.x1 && y <= self.y1 {
                // println!("{},{}", dx, dy);
                return true;
            }
            if ddx > 0 {
                x += ddx;
            }
            y += ddy;
            ddy -= 1;
            ddx -= 1;
        }
    }
}

pub fn input_generator(input: &str) -> Target {
    let mut space = input.split_whitespace();
    space.next().unwrap();
    space.next().unwrap();
    let mut x = space.next().unwrap();
    println!("{}", &x[2..x.len()-1]);
    let mut tmp = x[2..x.len()-1].split("..");
    let (x1, x2) = (tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap(),
        tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap());
    x = space.next().unwrap();
    tmp = x[2..].split("..");
    let (y1, y2) = (tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap(),
        tmp.next().map(|f|i32::from_str(f).unwrap()).unwrap());

    let xx1 = x1.min(x2);
    let xx2 = x1.max(x2);
    let yy1 = y1.max(y2);
    let yy2 = y1.min(y2);
    Target::new(xx1,yy1,xx2,yy2)
}

pub fn solve_part1(input: &Target) -> i32 {
    let mut max = 0;
    let mut dy = 0;
    for _ in 0..input.y2.abs() {
        let mut y = 0;
        let mut ddy = dy;
        let mut tmpmax = 0;
        loop {
            y += ddy;
            ddy -= 1;
            if y > tmpmax {
                tmpmax = y;
            }
            if y < input.y2 {
                break;
            } else if y <= input.y1 {
                break;
            }
        }

        max = tmpmax;
        dy += 1;
    }
    max
}

fn find_all_y(input: &Target) -> Vec<i32> {
    let mut dy = -1;
    let mut v = vec!();
    for _ in 0..input.y2.abs() {
        let mut y = 0;
        let mut ddy = dy;
        loop {
            y += ddy;
            ddy -= 1;
            if y < input.y2 {
                break;
            } else if y <= input.y1 {
                v.push(dy);
                v.push(dy.abs() -1);
                break;
            }
        }
        dy -= 1;
    }
    // println!("{:?}", v);
    v
}

fn find_all_x(input: &Target) -> Vec<i32> {
    let mut v = vec!();
    let mut dx = 0;
    for _ in 0..=input.x2 {
        let mut x = 0;
        let mut ddx = dx;
        loop {
            x += ddx;
            ddx -= 1;
            if x > input.x2 {
                break;
            } else if x >= input.x1 {
                v.push(dx);
                break;
            }else if ddx <= 0 {
                break;
            }
        }
        dx += 1;
    }
    v
}
pub fn solve_part2(input: &Target) -> usize {
    let dxs = find_all_x(input);
    let dys = find_all_y(input);
    let mut c = 0;
    for x in dxs {
        for y in &dys {
            if input.hit(x,*y) {
                c += 1
            }
        }
    }
    c
}
