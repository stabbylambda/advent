use crate::parsing::parse;

pub mod parsing;

use common::{answer, read_input};

fn main() {
    let input = read_input!();
    let input = parse(input);

    answer!(problem1(&input));
    answer!(problem2(&input));
}

type Input = Packet;
type Version = u8;
#[derive(Debug, PartialEq, Eq)]
pub enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Literal(Version, u64),
    Operator(Version, PacketType, Vec<Packet>),
}

impl Packet {
    fn get_version_sum(&self) -> u32 {
        match self {
            Packet::Literal(v, _) => *v as u32,
            Packet::Operator(v, _, sub) => sub
                .iter()
                .fold(*v as u32, |acc, s| acc + s.get_version_sum()),
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            Packet::Literal(_, v) => *v,
            Packet::Operator(_, t, sub) => {
                let mut results = sub.iter().map(|x| x.evaluate());
                match t {
                    PacketType::Sum => results.sum(),
                    PacketType::Product => results.product(),
                    PacketType::Minimum => results.min().unwrap(),
                    PacketType::Maximum => results.max().unwrap(),
                    PacketType::GreaterThan => {
                        let x = results.next().unwrap();
                        let y = results.next().unwrap();

                        (x > y) as u64
                    }
                    PacketType::LessThan => {
                        let x = results.next().unwrap();
                        let y = results.next().unwrap();

                        (x < y) as u64
                    }
                    PacketType::EqualTo => {
                        let x = results.next().unwrap();
                        let y = results.next().unwrap();

                        (x == y) as u64
                    }
                }
            }
        }
    }
}

fn problem1(input: &Input) -> u32 {
    input.get_version_sum()
}

fn problem2(input: &Input) -> u64 {
    input.evaluate()
}

#[cfg(test)]
mod test {
    use crate::parse;

    #[test]
    fn sums() {
        let result = parse("8A004A801A8002F478");
        assert_eq!(result.get_version_sum(), 16);
        let result = parse("620080001611562C8802118E34");
        assert_eq!(result.get_version_sum(), 12);
        let result = parse("C0015000016115A2E0802F182340");
        assert_eq!(result.get_version_sum(), 23);
        let result = parse("A0016C880162017C3686B18A3D4780");
        assert_eq!(result.get_version_sum(), 31);
    }

    #[test]
    fn evals() {
        let tests = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (input, expected) in tests {
            let result = parse(input);
            assert_eq!(result.evaluate(), expected);
        }
    }
}
