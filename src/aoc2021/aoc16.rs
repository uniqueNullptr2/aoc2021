use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
pub enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq
}
impl From<u64> for PacketType {
    fn from(i: u64) -> Self {
        match i {
            4 => PacketType::Literal,
            0=> PacketType::Sum,
            1=> PacketType::Product,
            2=> PacketType::Min,
            3=> PacketType::Max,
            5=> PacketType::Gt,
            6=> PacketType::Lt,
            7=> PacketType::Eq,
            _ => PacketType::Sum,
        }
    }
}
pub struct BitStream {
    inner: Vec<u8>,
    byte: usize,
    bit: usize,
}

impl BitStream {
    pub fn new(input: &str) -> Self {
        let inner = input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .chunks(2)
        .into_iter()
        .map(|mut ch| {
            let c1 = ch.next().unwrap();
            let c2 = ch.next().unwrap();
            (c1<<4)|c2
        })
        .collect();
        Self{inner, byte: 0, bit: 0}
    }

    pub fn read(&mut self, n: usize) -> u64 {
        let mut r = 0;
        for _ in 0..n {
            let b = (self.inner[self.byte] >> (7-self.bit)) & 1;
            r = (r << 1) + b as u64;
            self.bit += 1;
            if self.bit > 7 {
                self.bit -= 8;
                self.byte += 1;
            }
        }
        r
    }
}
#[derive(Clone, Debug)]
pub struct Packet {
    p_type: PacketType,
    ver: u64,
    packets: Option<Vec<Packet>>,
    literal: Option<u64>,
}

impl Packet {
    pub fn parse(bits: &mut BitStream) -> (usize, Self) {
        let ver = bits.read(3);
        let p_type = bits.read(3).into();
        match p_type {
            PacketType::Literal => {
                let (lit_len, lit) = Packet::parse_literal(bits);
                (6+lit_len, Self{ver, p_type, packets: None, literal: Some(lit)})
            },
            _ => {
                let (pac_len, pac) = Packet::parse_packets(bits);
                (6+pac_len, Self{ver, p_type, packets: Some(pac), literal: None})
            }
        }
    }

    pub fn sum_versions(&self) -> u64 {
        match self.p_type {
            PacketType::Literal => self.ver,
            _ => self.ver + self.packets.as_ref().unwrap().iter().map(|p| p.sum_versions()).sum::<u64>(),
        }
    }

    pub fn evaluate(&self) -> u64 {
        match self.p_type {
            PacketType::Literal => self.literal.unwrap(),
            PacketType::Sum => self.packets.as_ref().unwrap().iter().map(|p| p.evaluate()).sum::<u64>(),
            PacketType::Product => self.packets.as_ref().unwrap().iter().map(|p| p.evaluate()).product::<u64>(),
            PacketType::Min => self.packets.as_ref().unwrap().iter().map(|p| p.evaluate()).min().unwrap(),
            PacketType::Max => self.packets.as_ref().unwrap().iter().map(|p| p.evaluate()).max().unwrap(),
            PacketType::Eq => {
                let p = self.packets.as_ref().unwrap();
                (p[0].evaluate() == p[1].evaluate()) as u64
            },
            PacketType::Gt => {
                let p = self.packets.as_ref().unwrap();
                (p[0].evaluate() > p[1].evaluate()) as u64
            },
            PacketType::Lt => {
                let p = self.packets.as_ref().unwrap();
                (p[0].evaluate() < p[1].evaluate()) as u64
            }
        }
    }
    // pub fn packets(&self) -> Option<&Vec<Packet>> {
    //     self.subs.as_ref()
    // }

    fn parse_literal(bits: &mut BitStream) -> (usize, u64) {
        let mut n = 0;
        let mut c = 0;
        loop {
            let t = bits.read(5);
            c += 5;
            n = (n<<4) + (t&0b1111);
            if t & 0b10000 == 0 {
                break;
            }
        }
        (c,n)
    }

    fn parse_packets(bits: &mut BitStream) -> (usize, Vec<Packet>) {
        let b = bits.read(1)==1;
        let mut c = 1;
        let mut v = vec!();
        if b {
            let subs = bits.read(11);
            c += 11;
            for _ in 0..subs {
                let (cc, packet) = Packet::parse(bits);
                c += cc;
                v.push(packet);
            }
        } else {
            let bit_len = bits.read(15);
            c += 15;
            let mut cc = 0;
            loop {
                let (ccc, packet) = Packet::parse(bits);
                cc += ccc;
                v.push(packet);
                if cc == bit_len as usize {
                    c += cc;
                    break;
                }
            }
        }
        (c, v)
    }
}

pub fn input_generator(input: &str) -> Packet {
    let mut bits = BitStream::new(input);

    let (_,p) = Packet::parse(&mut bits);

    p
}


pub fn solve_part1(input: &Packet) -> u64 {
    input.sum_versions()
}



pub fn solve_part2(input: &Packet) -> u64 {
    input.evaluate()
}
